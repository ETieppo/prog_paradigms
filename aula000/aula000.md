
Parte A - Conceitos fundamentais

1. Por que estudar paradigmas?
Explique três benefícios práticos de estudar conceitos de linguagens de programação, mesmo quando o desenvolvedor trabalha diariamente com apenas uma linguagem.
Assim como a linguagem natural, as linguagens de programação tem suas formas de serem escritas, as frases que montamos podem ser construídas de formas diferentes, na programação não é tão diferente, os dialetos permitem essa maleabilidade, o que me leva ao primeiro ponto, saber qual paradigma escolher é também encontrar a forma mais adequada de resolver o problema, além disso acredito que facilita organizar e manter um padrão estrutural ou mesmo para compreensão de novas linguagens, visto que essas compartilham de conceitos, por fim, entender a forma mais adequada de construir também nos leva à escolha da linguagem/tecnologia ideal para o caso

2. Linguagem, paradigma e domínio
Defina com suas palavras: linguagem de programação, paradigma e domínio de aplicação. Depois, dê um exemplo que relacione os três conceitos.
A linguagem é quem torna o abstrato real, é onde as ideias deixam de ser ideias, essa por sua vez constrói o domínio, que é como o modelo e o negócio funcionam, ou seja, por que o software existe, qual problema é resolvido, e o paradigma é a conexão entre os dois, a forma estrutural aplicada na linguagem para resolver o problema

3. Sintaxe ou semântica?
Classifique cada situação como problema principalmente de sintaxe ou de semântica:
Situação
Classificação
a) Um parêntese foi aberto e não foi fechado.
sintaxe
b) O programa compila, mas deposita um valor negativo na conta errada.
semântica
c) Uma palavra reservada foi escrita incorretamente.
sintaxe
d) A condição do laço nunca se torna falsa.
semântica
e) Uma função recebe dois argumentos, mas a chamada fornece apenas um.
sintaxe
f) A fórmula da média usa a soma correta, mas divide pela quantidade errada.
semântica

Página 1
Paradigmas de Programação - Aula 000

1. Etapas de tradução
Organize as etapas abaixo em uma sequência coerente para um processo de compilação: geração de código, análise léxica, execução, análise sintática, análise semântica. Depois explique, em uma frase, a função de cada etapa.
léxica, sintática, semântica, geração de código, execução
Gera tokens; analisa estrutura; analisa significado e tipos; cria o executável; executa o código gerado pela etapa anterior
Parte B - Paradigmas em ação

2. Identificação de paradigmas
Associe cada descrição ao paradigma predominante:
Descrição
Paradigma
a) O programa é uma sequência de comandos que altera o estado.
imperativo
b) A solução é modelada com objetos que combinam dados e comportamentos.
POO
c) O resultado é obtido por composição e aplicação de funções.
funcional
d) O conhecimento é descrito por fatos e regras, e o sistema realiza inferência.
lógico
e) Várias tarefas progridem de forma sobreposta ou simultânea.
concorrente

3. Curto-circuito
Considere a condição:
x != 0 && 10 / x > 2
a) O que acontece quando x vale zero em uma linguagem com avaliação em curto-circuito? b) O que poderia acontecer se a linguagem avaliasse obrigatoriamente as duas partes? c) Reescreva a lógica usando uma estrutura condicional explícita.
se x for 0 a segunda etapa não é executada visto que a primeira já foi negativa; ao dividir 10/0 teríamos uma divisão que resulta em infinito, muitas vezes quebrando ou interrompendo o programa
if x != 0 {
 if 10/x < 2 {} else {}
}

4. Mesmo problema, estilos diferentes
Observe duas soluções em Python para somar os números pares de uma lista:

# Versão A

total = 0
for numero in numeros:
 if numero % 2 == 0:
 total = total + numero

# Versão B

total = sum(filter(lambda n: n % 2 == 0, numeros))
a) Qual versão apresenta estilo mais imperativo? Por quê?
versão A, altera o estado, qual a próxima linha a ser executada
b) Qual versão usa mais recursos funcionais?
 versao B, sum, filter, lambda
c) Qual delas você considera mais legível para uma equipe iniciante? Justifique.
 versão A, a segunda carrega o conceito de lambda, que por sua vez dificulta compreensão de escopo, além disso visualmente a primeira é mais organizada por ser explicitamente separada e em etapas

Parte C - Escolhas e análise crítica

1. Escopo e tempo de vida
Analise o pseudocódigo:
x = 10
funcao calcular():
 y = 5
 se x > y:
 z = x + y
 mostrar(z)
 mostrar(y)
 mostrar(z)
Assumindo escopo léxico convencional, indique onde x, y e z podem ser acessados. A última instrução é válida? Explique.
x foi definido fora do escopo da função, portanto pode ser acessada por ela, ja y dentro, esse somente por ela, z somente na condição, mostrar(z) pode ser incorreto em muitas linguagens

2. Escolha de linguagem
Uma equipe precisa desenvolver um pequeno sistema de análise de dados. O prazo é curto, a equipe já conhece Python e o desempenho exigido é moderado. Outra pessoa defende usar uma linguagem de baixo nível porque 'é mais rápida'. Avalie a decisão considerando pelo menos cinco critérios.
se desempenho exigido é moderado a escolha sensata seria o próprio python visto que o time tem conhecimento na mesma, além de a linguagem ter formas de otimizar, embora não seja tão rápida quanto as de baixo nível, possui uma biblioteca vasta para análise de dados, que por sua vez também carrega otimizações

3. Trade-offs
Explique por que não existe uma linguagem objetivamente melhor em todos os aspectos. Apresente dois exemplos de trade-off, como produtividade versus controle, segurança versus desempenho, simplicidade versus expressividade ou concisão versus legibilidade.
Rust é uma linguagem denominada segura, eficiente e veloz, mas também é uma das linguagens mais complexas do mundo, assim como o C, Python é simples e fácil e rápido de se escrever, prototipar mas também não tem tipos e pode ser lenta, não escala, Typescript roda tanto no navegador quanto em servidor, backend, tem tipagem, mas tem multi thread e concorrência ruins, carrega o v8 e pode ser extremamente pesado pra certos cenários, como embarcados por exemplo.
Parte D - Aplicação e síntese

4. Linguagem multiparadigma
Typescript é uma linguagem multi-paradigma
POO
class Pessoa {
 falar() {}
}

Funcional
function falar() {}

 1. Inteligência artificial e responsabilidade
verificar se existem arquivos que expõem chaves privadas da aplicação
leitura do código fonte e validação da regra de negócio e suas alterações
limpeza de código
analizar se o que foi gerado está dentro do padrão arquitetural definido
debug/build local
avaliar segurança e dependencias
executar casos de teste

 2. Desafio prático
Implemente, em qualquer linguagem, um programa que receba uma lista de números e produza: (1) a soma dos pares; (2) a quantidade de valores positivos; e (3) o maior valor. Depois identifique o paradigma predominante da sua solução e explique como ela poderia ser reescrita em outro estilo.
def analisar(numeros):
    soma_pares = 0
    positivos = 0
    maior = numeros[0]
    for n in numeros:
        if n % 2 == 0:
            soma_pares += n
        if n > 0:
            positivos += 1
        if n > maior:
            maior = n
    return soma_pares, positivos, maior

nums = [3, -2, 8, 5, -7, 4, 0]
sp, pos, mx = analisar(nums)
print(f"Soma dos pares: {sp}")
print(f"Positivos: {pos}")
print(f"Maior valor: {mx}")

ou
def analisar(numeros):
    soma_pares = sum(n for n in numeros if n % 2 == 0)
    positivos = sum(1 for n in numeros if n > 0)
    maior = max(numeros)
    return soma_pares, positivos, maior
