// Definição de componentes com e sem valor.
component Person
component Rock
component Name { value }

// Criação de entidades e instanciação de componentes.
entity(Person, Name { value: "John Doe" })
entity(Rock, Name { value: "Granite" })

// Definição de sistemas e queries simples.
system greet_people(name: Name, Person) {
    // Impressão de valores e acesso a campos de componentes.
    print("Hello,", name.value, "!")
}
