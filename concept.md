```rs

@(InverseOf, ChildOf)
component ParentOf

@(InverseOf, ParentOf)
component ChildOf


system add_inversion(
    (InverseOf, inverseRel) in directRel,
    (Added, directRel),
    target: RelationshipTarget in directRel,
    directRel in source,
) {
    target <- (inverseRel, source)
}

system remove_inversion(
    (InverseOf, inverseRel) in directRel,
    (Removing, directRel),
    target: RelationshipTarget in directRel,
    directRel in source,
) {
    target -> (inverseRel, source)
}



name: Name in player

(_, thing): (Likes, _) in player

system foo((tag, thing): (Likes | Hates | Loves, _) in player) {
    if tag == Likes { ... } else if ...
}

system foo(name: Name as nameE in e) {
    print("Entity {e} has Name {name}, where Name is entity {nameE}")
    nameE <- OtherComponent
}



// Phase 1:

system Foo(bar: Bar) {
    print(bar.baz, bar.qux)
}

component Bar {
    baz: Int,
    qux: String,
}

entity(Bar { baz: 1, qux: "a" })
