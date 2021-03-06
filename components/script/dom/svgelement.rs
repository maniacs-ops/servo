/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::SVGElementBinding;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::Root;
use dom::bindings::str::DOMString;
use dom::document::Document;
use dom::element::Element;
use dom::node::Node;
use dom::virtualmethods::VirtualMethods;
use html5ever_atoms::LocalName;
use style::element_state::ElementState;

#[dom_struct]
pub struct SVGElement {
    element: Element,
}

impl SVGElement {
    pub fn new_inherited(tag_name: LocalName, prefix: Option<DOMString>,
                         document: &Document) -> SVGElement {
        SVGElement::new_inherited_with_state(ElementState::empty(), tag_name, prefix, document)
    }

    pub fn new_inherited_with_state(state: ElementState, tag_name: LocalName,
                                    prefix: Option<DOMString>, document: &Document)
                                    -> SVGElement {
        SVGElement {
            element:
                Element::new_inherited_with_state(state, tag_name, ns!(svg), prefix, document),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(local_name: LocalName, prefix: Option<DOMString>, document: &Document) -> Root<SVGElement> {
        Node::reflect_node(box SVGElement::new_inherited(local_name, prefix, document),
                           document,
                           SVGElementBinding::Wrap)
    }
}

impl VirtualMethods for SVGElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<Element>() as &VirtualMethods)
    }
}
