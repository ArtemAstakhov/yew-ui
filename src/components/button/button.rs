use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink,
  Properties, Children, Callback, MouseEvent,
  Classes,
};
use yew::virtual_dom::{VNode, VList};
use crate::theme::{Theme, fade};

pub enum Msg {
  Click(MouseEvent),
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ButtonSize {
  Small,
  Medium,
  Large,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ButtonVariant {
  Contained,
  Outlined,
  Inline,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ButtonColor {
  Primary,
  Secondary,
  Error,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ButtonType {
  Submit,
  Button,
}

pub struct Button {
  style: Style,
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(ButtonVariant::Contained)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonColor::Primary)]
    pub color: ButtonColor,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
    #[prop_or(false)]
    pub fullwidth: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
}

fn get_style_class(variant: &ButtonVariant, color: &ButtonColor) -> String {
  let v = match variant {
      ButtonVariant::Contained => { "contained" }
      ButtonVariant::Outlined => { "outlined" }
      ButtonVariant::Inline => { "inline" }
  };
  let c = match color {
    ButtonColor::Primary => { "primary" }
    ButtonColor::Secondary => { "secondary" }
    ButtonColor::Error => { "error" }
  };

  format!("{c}-{v}", c = c, v = v)
}

fn get_size_class(size: &ButtonSize) -> String {
  let s = match size {
    ButtonSize::Small => { "small" }
    ButtonSize::Medium => { "medium" }
    ButtonSize::Large => { "large" }
  };

  format!("size-{}", s)
}

impl Component for Button {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let theme = Theme::default();
    let style = Style::create(
      String::from("button"),
      include_str!("button.scss")
        .replace("$button_border_radius", &theme.components.button.border_radius)
        .replace("$button_small_size_padding", &theme.components.button.small_size_padding)
        .replace("$button_large_size_padding", &theme.components.button.large_size_padding)
        .replace("$palette_primary_main_fade", &fade(theme.palette.primary.main.clone()))
        .replace("$palette_primary_main", &theme.palette.primary.main)
        .replace("$palette_primary_dark", &theme.palette.primary.dark)
        .replace("$palette_secondary_main_fade", &fade(theme.palette.secondary.main.clone()))
        .replace("$palette_secondary_main", &theme.palette.secondary.main)
        .replace("$palette_secondary_dark", &theme.palette.secondary.dark)
        .replace("$button_large_height", &theme.components.button.large_height)
        .replace("$button_small_height", &theme.components.button.small_height)
        .replace("$palette_error_main_fade", &fade(theme.palette.error.main.clone()))
        .replace("$palette_error_main", &theme.palette.error.main)
        .replace("$palette_error_dark", &theme.palette.error.dark),
    )
    .expect("An error occured while creating the style");

    Self {
      style,
      props,
      link,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Click(event) => {
        println!("Click");
        self.props.onclick.emit(event);
      }
    }
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.props != props {
      self.props = props;
      
      true
    } else {
        false
    }
  }

  fn view(&self) -> Html {
    let onclick = self.link.callback(Msg::Click);
    let class = self.format_classes();

    html! {
      <button
        class=class
        onclick=onclick
        disabled=self.props.disabled || self.props.loading
        type=self.get_type()
      >
        <div class="content">
          {self.props.children.clone()}
        </div>
        {self.render_spinner()}
      </button>
    }
  }
}

impl Button {
  fn format_classes(&self) -> Classes {
    let mut classes = Classes::from(self.style.clone().to_string());

    classes.push(self.props.class.clone());
    classes.push(get_style_class(&self.props.variant, &self.props.color));
    classes.push(get_size_class(&self.props.size));

    if self.props.disabled {
      classes.push("disabled");
    }
    if self.props.loading {
      classes.push("loading");
    }
    if self.props.fullwidth {
      classes.push("fullwidth");
    }

    classes
  }

  fn render_spinner(&self) -> Html {
    if self.props.loading {
      html! {
        <svg
          class="spinner"
          viewBox="0 0 50 50"
          data-ui-element="button-spinner"
        >
          <circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="5"></circle>
        </svg>
      }
    } else {
      VNode::from(VList::new())
    }
  }

  fn get_type(&self) -> String {
    match self.props.button_type {
      ButtonType::Button => { String::from("button") }
      ButtonType::Submit => { String::from("submit") }
    }
  }
}