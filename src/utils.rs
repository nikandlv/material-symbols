use inflector::Inflector;
use regex::Regex;

pub fn generate_tsx(icon_name: &str, theme: &str, svg_content: &str) -> String {
    let svg_body = extract_svg_body(svg_content);
    let svg_attributes = extract_svg_attributes(svg_content);

    format!(
        r#"import {{ SvgIcon, SvgIconProps }} from "@mui/material";

export default function Mui{}{}(props: SvgIconProps) {{
  return (
    <SvgIcon fill="none" {{...props}} {svg_attributes}>
      {svg_body}
    </SvgIcon>
  );
}}
"#,
        icon_name.to_pascal_case(),
        theme.to_pascal_case(),
        svg_attributes = svg_attributes,
        svg_body = svg_body
    )
}

pub fn extract_svg_body(svg_content: &str) -> String {
    let start = svg_content.find('>').unwrap() + 1;
    let end = svg_content.rfind("</svg>").unwrap();
    svg_content[start..end].to_string()
}

pub fn extract_svg_attributes(svg_content: &str) -> String {
    let svg_tag = svg_content.split('>').next().unwrap_or("").trim();
    let re = Regex::new(r#"([a-zA-Z\-]+)="([^"]+)""#).unwrap();
    let mut jsx_attributes = Vec::new();

    for cap in re.captures_iter(svg_tag) {
        let attribute = &cap[1];
        let value = &cap[2];
        if attribute != "width" && attribute != "height" && attribute != "xmlns" {
            let jsx_attribute = to_jsx_format(attribute);
            jsx_attributes.push(format!(r#"{jsx_attribute}="{}""#, value));
        }
    }

    jsx_attributes.join(" ")
}

pub fn to_jsx_format(attribute: &str) -> String {
    let parts: Vec<&str> = attribute.split('-').collect();
    let mut jsx_attr = parts[0].to_string();
    for part in parts.iter().skip(1) {
        jsx_attr.push_str(&part.to_pascal_case());
    }
    jsx_attr
}

pub fn capitalize_first_letter(s: &str) -> String {
    s.to_pascal_case()
}
