
1. Sintaxe e semântica
Explique “Sintaxe e semântica” com suas palavras e construa um exemplo curto que evidencie o conceito.
Fontes: presentation, slide 3, paraphrase

Sintaxe é a forma como é escrito, são as regras puras da linguagem e a qual sera analisada primeiro pelo compilador por busca de erros, como por exemplo não fechar '{}' ou esquecer um ';'; já a semântica é o sentido dado ao código escrito, como ele funciona, se soma x+y ou se divide.

1. Sentenças, lexemas e tokens
Lexema e token são a mesma coisa. Lexema é o que o dev escreve. O compilador lê isso e gera os tokens. Então soma e total são lexemas diferentes, mas caem no mesmo token. O token é a classe. O lexema é a instância dessa classe.

2. Reconhecedores e geradores
São dois papéis opostos. O reconhecedor recebe uma cadeia e responde se ela pertence ou não à linguagem. É o que o compilador faz na análise. O gerador faz o contrário. Ele parte das regras da gramática e produz cadeias válidas. A evidência é a direção. Cadeia entra e vem um sim ou não, é reconhecedor. Regra entra e saem cadeias, é gerador.

3. BNF e gramáticas livres de contexto
As duas descrevem a mesma coisa, mas em níveis diferentes. A GLC é o objeto matemático. São as regras de produção com não-terminais, terminais e símbolo inicial. A BNF é a notação que a gente usa para escrever essa gramática. Usa ::=, <> nos não-terminais e | nas alternativas. O critério que separa é esse. Uma é a teoria, a outra é a forma de anotar a teoria. O poder de expressão é o mesmo.

4. Derivação e árvore sintática
Serve para garantir que o código seja lido com a estrutura certa. Pega a + b * c. Você quer que a multiplicação venha antes. A derivação começa no símbolo inicial e vai aplicando as regras da gramática até chegar na cadeia. A árvore sintática mostra o resultado disso em forma de hierarquia. A multiplicação fica mais embaixo na árvore, então é resolvida primeiro. É assim que o compilador sabe a ordem das operações sem depender de parênteses.

5. Ambiguidade gramatical
Uma gramática é ambígua quando a mesma cadeia pode gerar duas árvores sintáticas diferentes. Isso é ruim porque o compilador fica sem saber qual sentido usar. Exemplo curto. Numa gramática solta, 2 - 3 - 4 pode agrupar como (2 - 3) - 4 ou como 2 - (3 - 4). Uma dá -5, a outra dá
6. Duas árvores para a mesma expressão. Isso é ambiguidade.

7. BNF estendida
O erro comum é achar que a EBNF é mais poderosa que a BNF. Não é. Ela só é mais confortável de escrever. A EBNF adiciona atalhos. Chaves para repetição, colchetes para parte opcional, parênteses para agrupar. Tudo isso já dava para fazer em BNF pura, só que com mais recursão e mais linhas. Versão corrigida: a EBNF não descreve nada que a BNF não descreva. Ela só deixa a gramática mais curta e legível.

8. Gramáticas de atributos
Serve para checar regras que a gramática livre de contexto sozinha não pega. Procedimento. Você pega um trecho e olha se existe alguma regra que depende de valor, tipo ou contexto. Um exemplo é a atribuição onde o tipo do lado direito precisa bater com o do lado esquerdo. A GLC aceita a forma x = y, mas não sabe checar tipo. A evidência é essa. Se a validação depende de significado e não só de forma, você está no terreno das gramáticas de atributos.

9. Atributos sintetizados e herdados
Os dois carregam informação pela árvore, mas em direções opostas. O atributo sintetizado sobe. Ele é calculado nos nós filhos e passado para o pai. É o caso do tipo de uma expressão, que vem de baixo. O atributo herdado desce. Ele vem do pai ou dos irmãos e é passado para o filho. É o caso de uma variável que precisa saber o tipo declarado lá em cima. O critério que separa é a direção do fluxo na árvore. Um sobe, o outro desce.

10. Métodos de semântica dinâmica
Serve para descrever o que o programa faz quando roda, não só a forma dele. Aplica bem no projeto de linguagem, quando você precisa definir o significado de um comando sem ambiguidade. Tem três abordagens. A semântica operacional descreve a execução passo a passo numa máquina abstrata. A semântica denotacional mapeia cada construção para um objeto matemático. A semântica axiomática usa lógica, com pré e pós-condição, e é a base da verificação formal de programas. A escolha depende do objetivo. Se você quer provar que o código está correto, a axiomática é a que encaixa.
