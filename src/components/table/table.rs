use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink, Properties, Classes, NodeRef,
  html::{
    ChildrenRenderer,
  },
  virtual_dom::{VComp, VChild},
};
use crate::theme::{Theme};
use crate::components::table::table_body::table_body::{TableBody, Props as TableBodyProps};
use crate::components::table::table_head::table_head::{TableHead, Props as TableHeadProps};

#[derive(Clone, PartialEq)]
pub enum Variants {
  Head(<TableHead as Component>::Properties),
  Body(<TableBody as Component>::Properties),
}

impl From<TableBodyProps> for Variants {
  fn from(props: TableBodyProps) -> Self {
      Variants::Body(props)
  }
}

impl From<TableHeadProps> for Variants {
  fn from(props: TableHeadProps) -> Self {
      Variants::Head(props)
  }
}

#[derive(PartialEq, Clone)]
pub struct ChildVariant {
  props: Variants,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum TableSize {
  Small,
  Medium,
}

pub struct Table {
  style: Style,
  props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<ChildVariant>,
    #[prop_or(TableSize::Medium)]
    pub size: TableSize,
}

impl<CHILD> From<VChild<CHILD>> for ChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<Variants>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl From<ChildVariant> for Html {
  fn from(variant: ChildVariant) -> Html {
      match variant.props {
        Variants::Head(props) => VComp::new::<TableHead>(props, NodeRef::default(), None).into(),
        Variants::Body(props) => VComp::new::<TableBody>(props, NodeRef::default(), None).into(),
      }
  }
}

impl Component for Table {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let theme = Theme::default();
    let style = Style::create(
      String::from("table"),
      include_str!("table.scss")
        .replace("$font_family", &theme.typography.font_family),
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
    html! {
      <table
        class=Classes::from(self.style.clone().to_string())
      >
        {
          self.props.children.iter()
            .filter(|c| matches!(c.props, Variants::Head(_)) || matches!(c.props, Variants::Body(_)))
            .map(|mut c| {
              if let Variants::Head(ref mut head_props) = c.props {
                head_props.size = self.props.size.clone();
              }
              if let Variants::Body(ref mut body_props) = c.props {
                body_props.size = self.props.size.clone();
              }
             
              c
            })
            .collect::<Html>()
        }
      </table>
    }
  }
}