use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink,
  Properties, Children, Callback, MouseEvent,
  Classes,
};
use crate::theme::{Theme};

pub enum Msg {
  Click(MouseEvent),
}

pub struct MenuItem {
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
    #[prop_or(false)]
    pub disabled: bool,
}

impl Component for MenuItem {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let theme = Theme::default();
    let style = Style::create(
      String::from("button"),
      include_str!("menu_item.scss")
        .replace("$breakpoint_md", &theme.breakpoints.md.to_string())
        .replace("$palette_divider", &theme.palette.divider),
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
      <div
        class=class
        onclick=onclick
      >
        {self.props.children.clone()}
      </div>
    }
  }
}

impl MenuItem {
  fn format_classes(&self) -> Classes {
    let mut classes = Classes::from(self.style.clone().to_string());

    classes.push(self.props.class.clone());

    if self.props.disabled {
      classes.push("disabled");
    }

    classes
  }
}