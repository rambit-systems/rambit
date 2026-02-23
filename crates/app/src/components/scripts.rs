use std::time::Duration;

use maud::{Markup, PreEscaped, html};

pub fn redirect_script(
  target: impl AsRef<str>,
  delay: Option<Duration>,
) -> Markup {
  let target = target.as_ref();
  let delay_ms = delay.map(|d| d.as_millis()).unwrap_or(0);

  let script_contents = PreEscaped(format!(
    "setTimeout(() => {{
      window.location.href = \"{target}\";
    }}, {delay_ms});"
  ));

  html! { script { (script_contents) } }
}
