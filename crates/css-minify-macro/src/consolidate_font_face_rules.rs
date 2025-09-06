use std::{cmp, collections::HashMap, hash};

use lightningcss::{
  properties::font::{AbsoluteFontWeight, FontWeight},
  rules::{
    CssRule, CssRuleList, Location,
    font_face::{FontFaceProperty, FontFaceRule, Source},
  },
  stylesheet::{PrinterOptions, StyleSheet},
  traits::ToCss,
  values::size::Size2D,
};
use proc_macro2::Span;
use syn::Error;

pub fn consolidate_font_face_rules(
  stylesheet: &mut StyleSheet,
  span: Span,
) -> Result<(), Error> {
  let rules = std::mem::take(&mut stylesheet.rules.0);
  let (font_face_rules, other_rules): (Vec<_>, Vec<_>) = rules
    .into_iter()
    .partition(|r| matches!(r, CssRule::FontFace(_)));

  let font_face_rules = font_face_rules.into_iter().filter_map(|r| match r {
    CssRule::FontFace(rule) => Some(rule),
    _ => None,
  });

  let mut prop_lists: HashMap<YokedSource, Vec<FontFaceProperty>> =
    HashMap::new();

  for rule in font_face_rules {
    let src = extract_first_source(rule.properties.clone().into_iter(), span)?;
    let remaining_properties =
      extract_non_source_properties(rule.properties.into_iter());

    prop_lists
      .entry(src)
      .or_default()
      .extend(remaining_properties);
  }

  let new_rules = prop_lists
    .into_iter()
    .map(|(yoked_source, properties)| {
      create_consolidated_font_face_rule(yoked_source, properties)
    })
    .collect::<Vec<_>>();

  stylesheet.rules =
    CssRuleList(other_rules.into_iter().chain(new_rules).collect());

  Ok(())
}

fn extract_first_source<'i, I: Iterator<Item = FontFaceProperty<'i>>>(
  mut properties: I,
  span: Span,
) -> Result<YokedSource<'i>, Error> {
  let source = properties
    .find_map(|p| match p {
      FontFaceProperty::Source(srcs) => Some(srcs),
      _ => None,
    })
    .ok_or_else(|| Error::new(span, "Font face rule missing src property"))?
    .first()
    .ok_or_else(|| Error::new(span, "Font face src property is empty"))?
    .clone();

  Ok(YokedSource::new(source))
}

fn extract_non_source_properties<
  'i,
  I: Iterator<Item = FontFaceProperty<'i>>,
>(
  properties: I,
) -> Vec<FontFaceProperty<'i>> {
  properties
    .filter(|p| !matches!(p, FontFaceProperty::Source(_)))
    .collect()
}

fn create_consolidated_font_face_rule<'a>(
  yoked_source: YokedSource<'a>,
  mut properties: Vec<FontFaceProperty<'a>>,
) -> CssRule<'a> {
  // Add the source back to properties
  properties.push(FontFaceProperty::Source(vec![yoked_source.into_inner()]));

  // Deduplicate properties
  let mut deduped_properties = Vec::new();
  for property in properties {
    if !deduped_properties.contains(&property) {
      deduped_properties.push(property);
    }
  }

  // Consolidate font-weight properties
  let consolidated_properties = consolidate_font_weights(deduped_properties);

  CssRule::FontFace(FontFaceRule {
    properties: consolidated_properties,
    loc:        Location {
      source_index: 0,
      line:         0,
      column:       0,
    },
  })
}

fn consolidate_font_weights(
  properties: Vec<FontFaceProperty>,
) -> Vec<FontFaceProperty> {
  let (font_weight_props, mut other_properties): (Vec<_>, Vec<_>) = properties
    .into_iter()
    .partition(|p| matches!(p, FontFaceProperty::FontWeight(_)));

  if font_weight_props.is_empty() {
    return other_properties;
  }

  let weight_values: Vec<f32> = font_weight_props
    .into_iter()
    .filter_map(|p| match p {
      FontFaceProperty::FontWeight(weight) => Some(weight),
      _ => None,
    })
    .flat_map(|w| [w.0, w.1])
    .filter_map(convert_to_numeric_weight)
    .collect();

  if let (Some(&min_weight), Some(&max_weight)) = (
    weight_values
      .iter()
      .min_by(|a, b| a.partial_cmp(b).unwrap()),
    weight_values
      .iter()
      .max_by(|a, b| a.partial_cmp(b).unwrap()),
  ) {
    let consolidated_weight = Size2D(
      FontWeight::Absolute(AbsoluteFontWeight::Weight(min_weight)),
      FontWeight::Absolute(AbsoluteFontWeight::Weight(max_weight)),
    );
    other_properties.push(FontFaceProperty::FontWeight(consolidated_weight));
  }

  other_properties
}

fn convert_to_numeric_weight(weight: FontWeight) -> Option<f32> {
  match weight {
    FontWeight::Absolute(AbsoluteFontWeight::Weight(w)) => Some(w),
    FontWeight::Absolute(AbsoluteFontWeight::Normal) => Some(400.0),
    FontWeight::Absolute(AbsoluteFontWeight::Bold) => Some(700.0),
    FontWeight::Bolder | FontWeight::Lighter => None,
  }
}

/// A [`Source`] yoked with its string representation, for use in hashing.
struct YokedSource<'i>(String, Source<'i>);

impl<'i> YokedSource<'i> {
  fn new(src: Source<'i>) -> YokedSource<'i> {
    Self(
      src
        .to_css_string(PrinterOptions::default())
        .expect("failed to serialize src to string"),
      src,
    )
  }

  fn into_inner(self) -> Source<'i> { self.1 }
}

impl hash::Hash for YokedSource<'_> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) { self.0.hash(state) }
}

impl cmp::PartialEq for YokedSource<'_> {
  fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
}

impl cmp::Eq for YokedSource<'_> {}
