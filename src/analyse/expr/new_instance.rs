use parse::tree::NewInstance;
use analyse::scope::Scope;
use analyse::expr;

pub fn apply<'def>(
    new_instance: &mut NewInstance<'def>,
    scope: &mut Scope<'def>,
) {
    match new_instance.name_opt {
       Some(name) => new_instance.class_def = scope.find_class(name.fragment).map(|c|c.parse),
       None => (),
    };

    for arg in &mut new_instance.args {
       expr::apply(arg, scope);
    }
}
