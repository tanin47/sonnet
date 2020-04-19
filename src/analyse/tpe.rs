use parse::tree::{Type, Class, Expr};
use analyse::scope::Scope;
use std::borrow::Borrow;

pub fn apply<'def>(
    tpe: &Type<'def>,
    scope: &mut Scope<'def>
) {
    if tpe.def_opt.get().is_none() {
        tpe.def_opt.set(scope.find_class(tpe.span.unwrap().fragment).map(|c| c.parse));
    }
}


pub trait GetType<'def> {
    fn get_type(&self, scope: &Scope<'def>) -> &Class<'def>;
}

impl <'def> GetType<'def> for Expr<'def> {
    fn get_type(&self, scope: &Scope<'def>) -> &Class<'def> {
        match self {
            Expr::Identifier(i) => unsafe { &*i.def_opt.borrow().as_ref().unwrap().get_type() },
            Expr::MemberAccess(i) => unsafe { &*(&*i.def_opt.get().unwrap()).tpe.def_opt.get().unwrap() },
            Expr::NewInstance(i) => unsafe { &*i.def_opt.get().unwrap() },
            Expr::Int(i) => unsafe { &*scope.find_class("Int").unwrap().parse },
            Expr::String(i) => unsafe { &*scope.find_class("String").unwrap().parse },
            Expr::Char(i) => unsafe { &*scope.find_class("Char").unwrap().parse },
            Expr::NativeInt(i) => unsafe { &*scope.find_class("Native__Int").unwrap().parse },
            Expr::NativeString(i) => unsafe { &*scope.find_class("Native__String").unwrap().parse },
            Expr::NativeChar(i) => unsafe { &*scope.find_class("Native__Char").unwrap().parse },
            Expr::Invoke(i) => unsafe { &*(&*i.def_opt.get().unwrap()).return_type.def_opt.get().unwrap() },
            Expr::Assignment(i) => unsafe { &*i.tpe.get().unwrap() },
        }
    }
}