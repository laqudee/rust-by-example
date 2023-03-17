mod libs;

fn main() {
    libs::function::play();

    libs::implement::play();

    libs::r#trait::play();

    libs::bound::play();
    libs::bound::null_bound_play();

    libs::multiple_bounds::play();

    libs::where_play::play();

    libs::new_type::play();

    libs::trait_problem::play();

    libs::type_trait::play();
}
