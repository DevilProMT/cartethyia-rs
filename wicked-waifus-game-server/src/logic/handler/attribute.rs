use wicked_waifus_protocol::{AttributeChangedRequest, AttributeChangedResponse, EAttributeType, ErrorCode, FormationAttrRequest, FormationAttrResponse};

use crate::{logic::{ecs::component::ComponentContainer, thread_mgr::NetContext}, query_components};

pub fn on_attribute_changed_request(
    ctx: &mut NetContext,
    request: AttributeChangedRequest,
    response: &mut AttributeChangedResponse,
) {
    let world = ctx.world.get_mut_world_entity();
	let id = request.id;

	if let (Some(mut component),) = query_components!(world, id, Attribute) {
		for needs_editing in request.attributes {
			if let Ok(attr_type) = EAttributeType::try_from(needs_editing.attribute_type) {
				component.attr_map.get_mut(&attr_type).unwrap().0 = needs_editing.current_value;
				component.attr_map.get_mut(&attr_type).unwrap().1 = needs_editing.value_increment;
			} else {
				tracing::warn!("Attribute skipped!");
				continue;
			}
		}
		
		response.error_code = ErrorCode::Success.into()
	} else {
		response.error_code = ErrorCode::ErrEntityNotFound.into()
	};
}

pub fn on_formation_attr_request(
    ctx: &mut NetContext,
    request: FormationAttrRequest,
    response: &mut FormationAttrResponse,
) {
	let player = &mut ctx.player;
    let world = ctx.world.get_mut_world_entity();
	
	let formation = &player.formation_list[&player.cur_formation_id];

	for role_id in formation.role_ids.clone() {
		if let (Some(mut component),) = query_components!(world, role_id as i64, Attribute) {
			for needs_editing in &request.formation_attrs {
				if let Ok(attr_type) = EAttributeType::try_from(needs_editing.attr_id) {
				component.attr_map.get_mut(&attr_type).unwrap().0 = needs_editing.current_value;
				component.attr_map.get_mut(&attr_type).unwrap().1 = needs_editing.max_value;
				} else {
					tracing::warn!("Attribute skipped!");
					continue;
				}
			}
			response.error_code = ErrorCode::Success.into()
		} else {
			response.error_code = ErrorCode::ErrEntityNotFound.into()
		};
	}
}