#import "../../../abnt_udc.typ": todo, figura_legendada

===	Entity Component System

Entity Component System (ECS) é um padrão arquitetural baseado no design orientado a dados. Ele surgiu na área de desenvolvimento de jogos, onde há uma grande necessidade de otimização e atualizações frequentes no código. Com o passar do tempo, o padrão ECS começou a ser utilizado em outras áreas, como em simulações físicas @flightdynamics.

O padrão consiste na separação de dado e lógica de tal forma que as vários entidades da aplicação possam ser compostas de dados reutilizáveis e independentes @ecsfaq, com as funções sendo direcionadas aos dados, e não às entidades em si. Devido ao desacoplamento gerado por essa separação, o padrão ECS garante alta flexibilidade e modularidade, além do aumento de desempenho gerado pela melhor distribuição de dados na memória @ecsstorageinpics.

Neste projeto, o padrão ECS será um dos principais fundamentos para o design e implementação da linguagem de programação, já que o intuíto dela será abstrair ele.

==== Os Três Elementos Fundamentais do ECS

Com base na explicação de #cite(<ecsfaq>, form: "prose"), pode-se dizer que o ECS é separado em três elementos fundamentais: entidades, componentes e sistemas. Cada um desses elementos desempenha um papel específico na aplicação:

- Entidades: identificadores únicos que representam os vários conceitos de uma aplicação. Sozinhas, as entidades não contêm dados nem funcionalidade;
- Componentes: estruturas de dados que armazenam informações específicas. Uma entidade pode ter múltiplos componentes diferentes, definindo suas características;
- Sistemas: funções responsáveis por processar sobre entidades com um determinado conjunto de componentes — processo denominado querying.

#figura_legendada(
    [Relação entre entidades, componentes e sistemas.],
    image("../../../../images/diagrama_ecs.png", height: 35%),
    [Fonte: elaboração própria.],
)

Como a figura acima ilustra, o estado da aplicação é dado por um conjunto de entidades, cada uma com seus respectivos componentes. Os sistemas são responsáveis pela transformação do estado da aplicação, processando as entidades que possuem os componentes necessários para a execução do sistema.

Em termos de código, o padrão ECS pode ser representado sem nenhum construto especializado, mapeando entidades para números únicos, componentes para _structs_ e sistemas para funções:

#figura_legendada(
    [Implementação simplificada de um padrão ECS incompleto.],
    ```rs
    // Componentes podem ser representados através de simples structs.
    struct Position { x: f32, y: f32 }

    struct Velocity { dx: f32, dy: f32 }

    // Sistemas podem ser representados como funções que processam todas as
    // entidades e seus respectivos componentes.
    fn apply_velocity(
        entities: &[usize],
        positions: &mut [Position],
        velocities: &[Velocity]
    ) {
        for &entity in entities {
            positions[entity].x += velocities[entity].dx;
            positions[entity].y += velocities[entity].dy;
        }
    }

    fn main() {
        // Entidades podem ser representadas como números únicos.
        let entities = [0, 1];

        // Cada componente é armazenado em um vetor separado, onde o índice
        // representa a entidade e o valor representa seu componente.
        let mut positions = [
            Position { x: 0.0, y: 0.0 }, // Entidade 0.
            Position { x: 1.0, y: 1.0 }, // Entidade 1.
        ];

        let velocities = [
            Velocity { dx: 1.0, dy: 1.0 }, // Entidade 0.
            Velocity { dx: 2.0, dy: 2.0 }, // Entidade 1.
        ];

        // No padrão ECS, é muito comum que os sistemas sejam executados repetidamente.
        loop {
            apply_velocity(&entities, &mut positions, &velocities);
        }
    }
    ```,
    [Fonte: elaboração própria.],
)

É importante ressaltar que o código acima, por mais que seja funcional e siga o design orientado a dados, ainda é uma simplificação da implementação de um padrão ECS incompleto. Na prática, o armazenamento dos dados é feito através de estruturas de dados mais complexas @ecsstorageinpics, que permitem que entidades escolham quais componentes possuem, que sistemas sejam executados automaticamente, além de outras funcionalidades principais do padrão ECS.

Fora a definição de ECS e seus três elementos fundamentais, o padrão ainda peca pela falta de formalização — quais são as práticas recomendadas ao usar ECS? Como os sistemas são executados? E se apenas entidades, componentes e sistemas não forem suficientes para resolver meu problema? Essas perguntas não possuem respostas definitivas, porém, diferentes autores e implementações abordam o padrão do seu jeito. A seguir, são apresentados alguns conceitos herdados de tais autores e implementações.

==== Agendador

O agendador é um construto com a finalidade de executar todos os sistemas da aplicação, podendo determinar a ordem e frequência de execução de forma declarativa, resolvendo dependência entre sistemas e tornando o ciclo de atualização da aplicação mais previsível @bevy. Pode-se dizer que, dentre os conceitos mais experimentais, o agendador é o mais próximo de uma formalização.

#figura_legendada(
    [Agendador executando os sistemas de forma cíclica e sequencial.],
    image("../../../../images/diagrama_agendador.png", height: 30%),
    [Fonte: elaboração própria.],
)

==== Relacionamento de Entidades

Independente da aplicação, é muito comum a necessidade de relacionar diferentes conceitos entre si. Exemplo disso são os sistemas de arquivos, onde pastas podem conter arquivos, uma relação pai-filho.

Relacionamento de entidades (do inglês, _entity relationship_) é um conceito que supre essa necessidade, permitindo que entidades se relacionem entre si. O autor da biblioteca Flecs, #cite(<flecs>, form: "author"), explica o conceito fazendo uma paralela com o simples ato de adicionar um componente a uma entidade:

#figura_legendada(
    [Entidade com um componente de posição.],
    image("../../../../images/entidade_posicao.png", height: 6%),
    [```ts Entidade.set<Posição>({0, 0});```],
    [Fonte: #cite(<entityrelationships>, form: "prose").],
)

Do mesmo jeito que se adiciona um único componente a uma entidade, como mostra a figura acima, pode-se criar um relacionamento entre duas entidades adicionando uma tupla componente-entidade, onde o componente dita o tipo de relação:

#figura_legendada(
    [Representação do relacionamento entre o Sol, a Terra e a Lua.],
    image("../../../../images/relacionamento_entidades.png", height: 13%),
    [```cs Terra.add(FilhoDe, Sol); Lua.add(FilhoDe, Terra);```],
    [Fonte: adaptado de #cite(<entityrelationships>, form: "prose").],
)

==== Depurador de ECS

Um depurador especializado em ECS é uma interface, gráfica ou não, responsável por ajudar o desenvolvedor a encontrar e resolver bugs relacionados ao ECS na aplicação. Tal depurador pode visualizar todas as entidades da aplicação, além  de seus respectivos componentes. Ele também pode listar os sistemas em execução e permitir o controle sobre eles, como pausar ou alterar a ordem de execução.

#figura_legendada(
    [Flex Explorer, depurador visual para a biblioteca Flecs.],
    image("../../../../images/flecs_explorer.png", height: 30%),
    [Fonte: #cite(<flecsexplorer>, form: "prose").],
)
