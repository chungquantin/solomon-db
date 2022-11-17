use serde_json::Value;
use uuid::Uuid;

use crate::util::{build_bytes, Component};
use crate::{Edge, EdgePropertyController, Error, Identifier, SimpleTransaction};

impl_controller!(EdgeController("edges:v1"));

impl<'a> EdgeController<'a> {
	fn key(&self, out_id: Uuid, t: &Identifier, in_id: Uuid) -> Vec<u8> {
		build_bytes(&[Component::Uuid(out_id), Component::Identifier(t), Component::Uuid(in_id)])
			.unwrap()
	}

	pub async fn create(
		&self,
		source_vertex: Uuid,
		target_vertex: Uuid,
		t: &str,
		props: Value,
	) -> Result<Edge, Error> {
		let mut tx = self.get_ds().transaction(true).unwrap();

		let cf = self.get_cf();
		let t_id = Identifier::new(t.to_string()).unwrap();
		let key = self.key(target_vertex, &t_id, source_vertex);
		let epc = EdgePropertyController::new(self.ds_ref);
		let props = epc.create(source_vertex, &t_id, target_vertex, props).await.unwrap();

		tx.set(cf, key, []).await.unwrap();
		tx.commit().await.unwrap();

		let edge = Edge::new(source_vertex, target_vertex, t_id, props).unwrap();
		Ok(edge)
	}

	pub async fn get(
		&self,
		source_vertex: Uuid,
		target_vertex: Uuid,
		t: &str,
	) -> Result<Edge, Error> {
		let t_id = Identifier::new(t.to_string()).unwrap();
		let epc = EdgePropertyController::new(self.ds_ref);
		let props = epc.iterate_from_attributes(source_vertex, &t_id, target_vertex).await.unwrap();
		let edge = Edge::new(source_vertex, target_vertex, t_id, props).unwrap();
		Ok(edge)
	}
}