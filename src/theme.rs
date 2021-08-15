use std::cmp;
use radix_fmt::radix;
use regex::Regex;

pub struct Theme {
  pub components: ThemeComponents,
  pub palette: Palette,
  pub breakpoints: Breakpoints,
}

pub struct Palette {
  pub primary: PaletteColor,
  pub secondary: PaletteColor,
  pub error: PaletteColor,
  pub divider: String,
}

pub struct PaletteColor {
  pub main: String,
  pub light: String,
  pub lightest: String,
  pub dark: String,
  pub darkest: String,
}

pub struct Breakpoints {
  pub xs: i32,
  pub sm: i32,
  pub md: i32,
  pub lg: i32,
}

#[derive(Clone)]
pub struct RGB(i32, i32, i32);

pub struct Color {
  alpha: f64,
  rgb: RGB,
}

pub enum BlendMode {
  Multiply,
  Screen,
}

pub fn blend_color(
  mode: BlendMode,
  color_1: Color,
  color_2: Color,
) -> Color {
  let ab = color_1.alpha;
  let mut cb: f64;
  let a_s = color_2.alpha;
  let mut cs: f64;
  let mut cr;

  let ar = a_s + ab * (1.0 - a_s);

  let color_1_rgb = vec![color_1.rgb.0, color_1.rgb.1, color_1.rgb.2];
  let color_2_rgb = vec![color_2.rgb.0, color_2.rgb.1, color_2.rgb.2];
  let mut result_rgb: Vec<i32> = [].to_vec();

  for i in 0..3 {
    cb = color_1_rgb[i] as f64 / 255.0;
    cs = color_2_rgb[i] as f64 / 255.0;

    // log::info!("========");
    // log::info!("{}", color_1_rgb[i]);
    // log::info!("{}", color_2_rgb[i]);

    // log::info!("cb: {}", cb);
    // log::info!("cs: {}", cs);
    
    match mode {
      BlendMode::Multiply => {
        cr = cb * cs
      }
      BlendMode::Screen => {
        cr = cb + cs - cb * cs;
      }
    }

    // log::info!("cr: {}", cr);

    if ar != 0.0 {
      cr = (a_s * cs + ab * (cb - a_s * (cb + cs - cr))) / ar;
    }

    // log::info!("{}", (cr * 255.0) as i32);
    result_rgb.push((cr * 255.0) as i32);
  }

  return generate_color_from_rgb(&RGB(result_rgb[0], result_rgb[1], result_rgb[2]), Some(ar));
}

pub fn generate_color_from_rgb(rgb: &RGB, alpha: Option<f64>) -> Color {
  Color {
    alpha: alpha.unwrap_or(1.0),
    rgb: rgb.clone(),
  }
}

fn clamp(v: i32, max: i32) -> i32 {
  cmp::min(cmp::max(v, 0), max)
}

fn to_hex(v: &RGB) -> String {
  let vector = vec![v.0, v.1, v.2];

  let hex: Vec<String> = vector.into_iter().map(|c| {
    let clamped = clamp(c, 255);
    let prefix = if clamped < 16 { "0" } else { "" };
    
    format!("{p}{r}", p = prefix, r = radix(clamped, 16))
  }).collect();

  format!("#{}", hex.join(""))
}

fn hex_to_rgb(hex: String) -> RGB {
  let re = Regex::new(r#"^#?(?P<one>[a-f\d]{2})(?P<two>[a-f\d]{2})(?P<three>[a-f\d]{2})$"#).unwrap();
  let caps = re.captures(&hex).expect("Invalid hex string");

  RGB(
    i32::from_str_radix(&caps["one"], 16).unwrap(),
    i32::from_str_radix(&caps["two"], 16).unwrap(),
    i32::from_str_radix(&caps["three"], 16).unwrap(),
  )
}

pub fn fade(color: String) -> String {
  let rgb = hex_to_rgb(color);
  let c = generate_color_from_rgb(&rgb, None);

  format!("rgba({}, {}, {}, .05)", c.rgb.0, c.rgb.1, c.rgb.2)
}

pub fn generate_palette(color: RGB) -> PaletteColor {
  let l1_additive: RGB = hex_to_rgb(String::from("#464646"));
  let l2_additive: RGB = hex_to_rgb(String::from("#898989"));
  // const l3Additive = hexToRgb("#b8b8b8");
  // const l4Additive = hexToRgb("#dadada");
  // const l5Additive = hexToRgb("#f0f0f0");
  let d1_additive: RGB = hex_to_rgb(String::from("#e1e1e1"));
  let d2_additive: RGB = hex_to_rgb(String::from("#c2c2c2"));
  // const d3Additive = hexToRgb("#9b9b9b");
  // const d4Additive = hexToRgb("#727272");
  // const d5Additive = hexToRgb("#4c4c4c");

  PaletteColor {
    main: to_hex(&color),
    dark: to_hex(&blend_color(
      BlendMode::Multiply,
      generate_color_from_rgb(&color, None),
      generate_color_from_rgb(&d1_additive, None),
    ).rgb),
    darkest: to_hex(&blend_color(
      BlendMode::Multiply,
      generate_color_from_rgb(&color, None),
      generate_color_from_rgb(&d2_additive, None),
    ).rgb),
    light: to_hex(&blend_color(
      BlendMode::Screen,
      generate_color_from_rgb(&color, None),
      generate_color_from_rgb(&l1_additive, None),
    ).rgb),
    lightest: to_hex(&blend_color(
      BlendMode::Screen,
      generate_color_from_rgb(&color, None),
      generate_color_from_rgb(&l2_additive, None),
    ).rgb),
  }
}

pub struct ThemeComponents {
  pub button: ComponentButton,
}

pub struct ComponentButton {
  pub border_radius: String,
  pub small_size_padding: String,
  pub large_size_padding: String,
  pub small_height: String,
  pub large_height: String,
}

impl Theme {
  // pub fn new(
  //   components: ThemeComponents,
  //   palette: Palette,
  // ) -> Self {
  //   Theme {
  //     components,
  //     palette,
  //   }
  // }

  pub fn default() -> Self {
    Theme {
      components: Theme::get_default_components(),
      palette: Theme::get_default_palette(),
      breakpoints: Theme::get_default_breakpoints(),
    }
  }

  pub fn get_default_palette() -> Palette {
    Palette {
      primary: generate_palette(RGB(13, 183, 182)),
      secondary: generate_palette(RGB(234, 40, 69)),
      error: generate_palette(RGB(244, 67, 54)),
      divider: String::from("#DDE3ED"),
    }
  }

  pub fn get_default_components() -> ThemeComponents {
    ThemeComponents {
      button: Theme::get_default_component_button(),
    }
  }

  pub fn get_default_component_button() -> ComponentButton {
    ComponentButton {
      border_radius: String::from("4px"),
      small_size_padding: String::from("10px 16px"),
      large_size_padding: String::from("12px 32px"),
      small_height: String::from("36px"),
      large_height: String::from("43px"),
    }
  }

  pub fn get_default_breakpoints() -> Breakpoints {
    Breakpoints {
      xs: 320,
      sm: 480,
      md: 768,
      lg: 1000,
    }
  }
}