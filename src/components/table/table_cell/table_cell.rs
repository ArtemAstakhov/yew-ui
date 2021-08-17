use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink,
  Properties, Children, Classes,
};
use crate::components::table::TableSize;
use crate::theme::{Theme};

#[derive(Clone, PartialEq, Debug)]
pub enum TableCellVariant {
  Head,
  Body,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum TableCellAlign {
  Right,
  Left,
  Center,
}

pub struct TableCell {
  style: Style,
  props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(TableCellAlign::Left)]
    pub align: TableCellAlign,
    #[prop_or_default]
    pub variant: Option<TableCellVariant>,
    #[prop_or(TableSize::Medium)]
    pub size: TableSize,

}

fn get_align_class(size: &TableCellAlign) -> String {
  let s = match size {
    TableCellAlign::Left => { "left" }
    TableCellAlign::Right => { "right" }
    TableCellAlign::Center => { "center" }
  };

  format!("align-{}", s)
}

fn get_size_class(size: &TableSize) -> String {
  let s = match size {
    TableSize::Small => { "small" }
    TableSize::Medium => { "medium" }
  };

  format!("size-{}", s)
}

impl Component for TableCell {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let theme = Theme::default();
    let style = Style::create(
      String::from("table_cell"),
      include_str!("table_cell.scss")
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
    let component = if let Some(v) = &self.props.variant {
      match v {
          TableCellVariant::Body => { "td" }
          TableCellVariant::Head => { "th" }
      }
    } else {
      "td"
    };

    html! {
      <@{component}
        class=class
      >
        {self.props.children.clone()}
      </@>
    }
  }
}

impl TableCell {
  fn format_classes(&self) -> Classes {
    let mut classes = Classes::from(self.style.clone().to_string());

    classes.push(self.props.class.clone());
    classes.push(get_size_class(&self.props.size));
    classes.push(get_align_class(&self.props.align));

    classes
  }
}