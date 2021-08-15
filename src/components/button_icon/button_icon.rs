use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink,
  Properties, Children, Classes,
};
use crate::theme::{Theme};

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ButtonIconSize {
  Small,
  Medium,
  Large,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ButtonIconPosition {
  Start,
  End,
}

pub struct ButtonIcon {
  style: Style,
  props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(ButtonIconSize::Medium)]
    pub size: ButtonIconSize,
    #[prop_or(ButtonIconPosition::Start)]
    pub position: ButtonIconPosition,
}

fn get_size_class(size: &ButtonIconSize) -> String {
  let s = match size {
    ButtonIconSize::Small => { "small" }
    ButtonIconSize::Medium => { "medium" }
    ButtonIconSize::Large => { "large" }
  };

  format!("size-{}", s)
}

fn get_position_class(size: &ButtonIconPosition) -> String {
  let s = match size {
    ButtonIconPosition::Start => { "start" }
    ButtonIconPosition::End => { "end" }
  };

  format!("position-{}", s)
}

impl Component for ButtonIcon {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let theme = Theme::default();
    let style = Style::create(
      String::from("button"),
      include_str!("button_icon.scss")
        .replace("$breakpoint_md", &theme.breakpoints.md.to_string())
        .replace("$palette_divider", &theme.palette.divider),
    )
    .expect("An error occured while creating the style");

    Self {
      style,
      props,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
    let class = self.format_classes();

    html! {
      <div
        class=class
      >
        {self.props.children.clone()}
      </div>
    }
  }
}

impl ButtonIcon {
  fn format_classes(&self) -> Classes {
    let mut classes = Classes::from(self.style.clone().to_string());

    classes.push(self.props.class.clone());
    classes.push(get_size_class(&self.props.size));
    classes.push(get_position_class(&self.props.position));

    classes
  }
}