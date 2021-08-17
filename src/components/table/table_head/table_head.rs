use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink,
  Properties, Classes, NodeRef,
  html::{
    ChildrenRenderer,
  }
};
use yew::virtual_dom::{VChild, VComp};
use crate::components::table::table_row::table_row::{
  TableRow,
  Props as TableRowProps,
};
use crate::components::table::TableSize;
use crate::components::table::table_cell::table_cell::TableCellVariant;

#[derive(Clone, PartialEq)]
pub enum Variants {
  Row(<TableRow as Component>::Properties),
}

impl From<TableRowProps> for Variants {
  fn from(props: TableRowProps) -> Self {
      Variants::Row(props)
  }
}

#[derive(PartialEq, Clone)]
pub struct ChildVariant {
  props: Variants,
}

pub struct TableHead {
  style: Style,
  props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<ChildVariant>,
    #[prop_or(TableSize::Small)]
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
        Variants::Row(props) => VComp::new::<TableRow>(props, NodeRef::default(), None).into(),
      }
  }
}

impl Component for TableHead {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let style = Style::create(
      String::from("table_head"),
      include_str!("table_head.scss"),
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
      <thead
        class=Classes::from(self.style.clone().to_string())
      >
        {
          self.props.children.iter()
            .filter(|c| matches!(c.props, Variants::Row(_)))
            .map(|mut c| {
              let Variants::Row(ref mut props) = c.props;
              props.variant = Some(TableCellVariant::Head);
              props.size = self.props.size.clone();
              c
            })
            .collect::<Html>()
        }
      </thead>
    }
  }
}