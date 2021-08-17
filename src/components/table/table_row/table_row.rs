use css_in_rust::Style;
use yew::{
  html, Component, ShouldRender, Html, ComponentLink, Properties, Classes, NodeRef,
  html::{
    ChildrenRenderer,
  },
  virtual_dom::{
    VComp, VChild,
  }
};
use crate::components::table::TableSize;
use crate::components::table::table_cell::table_cell::{
  TableCell,
  Props as TableCellProps,
  TableCellVariant,
};

#[derive(Clone, PartialEq)]
pub enum Variants {
  Cell(<TableCell as Component>::Properties),
}

impl From<TableCellProps> for Variants {
  fn from(props: TableCellProps) -> Self {
      Variants::Cell(props)
  }
}

#[derive(PartialEq, Clone)]
pub struct ChildVariant {
    props: Variants,
}

pub struct TableRow {
  style: Style,
  props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<ChildVariant>,
    #[prop_or_default]
    pub variant: Option<TableCellVariant>,
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
        Variants::Cell(props) => VComp::new::<TableCell>(props, NodeRef::default(), None).into(),
      }
  }
}

impl Component for TableRow {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let style = Style::create(
      String::from("table_row"),
      include_str!("table_row.scss"),
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
      <tr
        class=Classes::from(self.style.clone().to_string())
      >
        {
          self.props.children.iter()
            .filter(|c| matches!(c.props, Variants::Cell(_)))
            .map(|mut c| {
              let Variants::Cell(ref mut props) = c.props;
              props.variant = self.props.variant.clone();
              props.size = self.props.size.clone();
              c
            })
            .collect::<Html>()
        }
      </tr>
    }
  }
}