use crate::*;

pub struct EntityPathJar(
    <PackagePath as salsa::storage::IngredientsFor>::Ingredients,
    <EntityPath as salsa::storage::IngredientsFor>::Ingredients,
    pub(crate) EntityPathMenuPlace,
);

impl salsa::storage::HasIngredientsFor<PackagePath> for EntityPathJar {
    fn ingredient(&self) -> &<PackagePath as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <PackagePath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}

impl salsa::storage::HasIngredientsFor<EntityPath> for EntityPathJar {
    fn ingredient(&self) -> &<EntityPath as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <EntityPath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for EntityPathJar {
    type DynDb = dyn EntityPathDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <PackagePath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <EntityPath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, Default::default())
    }
}