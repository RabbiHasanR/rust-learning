mod shared_references;
mod exclusive_references;
mod slices;
mod strings;
mod reference_validity;
mod geometry;

fn main() {
    shared_references::shared_reference();
    exclusive_references::exclusive_reference();
    slices::slice();
    strings::strings();
    strings::string_one();
    strings::string_two();
    reference_validity::reference_valid();
    geometry::geometry();
}
