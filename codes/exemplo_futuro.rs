// Torna impossível adicionar `Flying` a uma entidade que já tenha `Grounded`.
@(Exclusive, Grounded)
component Flying

// Torna impossível adicionar `Grounded` a uma entidade que já tenha `Flying`.
// Com isso, os componentes se tornam mutualmente exclusivos.
@(Exclusive, Flying)
component Grounded

// Diz que o sistema `fly` deve ser executado após `some_other_system`.
@(RunAfter, some_other_system)
system fly() { ... }