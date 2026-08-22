
1. Razões para estudar conceitos de LP
Na escolha da equipe, estudar esses conceitos permite avaliar as linguagens candidatas por critérios objetivos, evita retrabalho e escolher tecnologias que não são ideais para o caso

2. Domínios de Programação
O raciocínio ignora que "funcionar hoje" não garante adequação ao domínio, que impõe requisitos distintos de desempenho, precisão e bibliotecas. mesmo funcionando, o código pode estar no domínio errado, gerando ineficiência e dificuldade de evolução.

3. Critérios de Avaliação 
Legibilidade, capacidade de escrita e confiabilidade de forma estruturada. Manutenção consome a maior parte do custo do software, então código mais legível reduz esse custo diretamente.

4. Legibilidade  
A, faz mais sentido em equipes grandes e projetos de longa manutenção. B muitos operadores e atalhos densos, faz sentido em scripts curtos e descartáveis.

5. Ortogonalidade  
Definir a linguagem de um sistema com muitos tipos e operações combinadas. ortogonalidade, poucos elementos primitivos combináveis livremente e sem exceções. porém, mais ortogonalidade dá regularidade e menos casos especiais, mas excesso pode permitir combinações válidas porém sem sentido, dificultando a detecção de erros.

6. Capacidade de escrita 
Entra medindo quão facilmente a linguagem expressa a solução do problema. como consequência escolher uma linguagem com boa capacidade de escrita para o domínio reduz linhas de código e tempo de desenvolvimento; escolher mal aumenta esforço e chance de erro.

7. Suporte para abstração  
O raciocínio ignora que sem abstração o código repete lógica e fica difícil de estender, mesmo funcionando. Código que funciona mas não abstrai, subprogramas, tipos, estruturas de dados, tende a crescer duplicado e caro de manter.

8. Expressividade 
Verificar se a linguagem tem formas convenientes de expressar computaçõe. resultando em construções mais expressivas que reduzem código e erros, como operadores compostos frente a formas verbosas equivalentes.

9. Confiabilidade 
Alternativa com verificação de tipos forte e tratamento de exceções faz sentido em sistemas críticos. Alternativa mais permissiva faz sentido em protótipos e experimentação, onde a flexibilidade vale mais que garantias em tempo de execução.

10. Verificação de tipos 
Sistema onde erros silenciosos são caros. verificação em tempo de compilação. Trade-off: pegar erros cedo aumenta a confiabilidade e reduz falhas em produção, ao custo de menos flexibilidade e mais rigidez para o programador.

11. Manipulação de exceções 
Entra avaliando se a linguagem permite tratar erros em tempo de execução de forma estruturada. Uma linguagem com bom suporte a exceções gera sistemas mais robustos que se recuperam de falhas; sem isso, erros tendem a derrubar a aplicação.

12. Aliasing
O raciocínio ignora que dois nomes apontando para a mesma célula de memória geram efeitos colaterais difíceis de rastrear. aliasing reduz legibilidade e confiabilidade e pode causar bugs sutis quando o código for alterado.

13. Custo 
treinamento, escrita, compilação, execução, manutenção e confiabilidade. custo de manutenção sobre o ciclo de vida. a manutenção domina o custo do software, então uma linguagem de execução barata mas ilegível pode sair mais cara no total.

14. Portabilidade e generalidade 
Uma linguagem portável faz sentido quando o sistema roda em várias plataformas. Uma linguagem específica de plataforma faz sentido quando se busca máximo desempenho num ambiente único e o software não migrará.

15. Influências no Projeto 
Entender por que a linguagem tem certas restrições. Arquitetura da máquina e metodologias de projeto de software como forças que moldam a linguagem. Trade-off: linguagens alinhadas à arquitetura ganham desempenho, mas podem afastar-se de paradigmas mais abstratos como o funcional.

16. Arquitetura de von Neumann 
Explica que a maioria das linguagens imperativas reflete esse modelo (variáveis = memória, atribuição = transferência de dados). Leva a escolher uma linguagem imperativa que aproveita bem essa arquitetura; um paradigma muito distante dela pode ter penalidade de desempenho.

17. Ciclo busca-decodifica-executa 
O raciocínio ignora que esse ciclo explica o gargalo de von Neumann, o que afeta desempenho. Entender o ciclo ajuda a escrever código eficiente e a compreender limites da máquina.

18. Metodologias de Projeto de Software 
Avaliar se a linguagem apoia a metodologia adotada. A aderência da linguagem à metodologia do projeto. A evolução das metodologias historicamente puxou novos recursos de linguagem.

19. Categorias de Linguagens 
Uma linguagem imperativa/OO faz sentido em sistemas de negócio grandes e mutáveis. Uma linguagem funcional faz sentido em problemas com forte base matemática, concorrência e transformação de dados, onde imutabilidade ajuda.

20. Trade-off no projeto Definir prioridade de projeto da linguagem. Confiabilidade versus custo de execução. mais checagens aumentam a segurança mas reduzem a velocidade; menos checagens são rápidas mas arriscadas.

21. Métodos de Implementação 
Considera se a linguagem é compilada, interpretada ou híbrida, o que afeta desempenho e portabilidade. escolher uma compilada favorece velocidade de execução; uma interpretada favorece portabilidade e desenvolvimento rápido.

22. Compilação 
O raciocínio ignora que a compilação afeta tempo de tradução, desempenho de execução e detecção antecipada de erros. A compilação importa para decidir entre velocidade de execução ou no desenvolvimento.
23. Interpretação pura 
Avaliar se a agilidade e a facilidade de depuração compensam a perda de desempenho. Velocidade de execução X flexibilidade em desenvolvimento. Interpretação pura é bem mais lenta, então só se justifica onde a produtividade importa mais que a velocidade.

24. Sistemas Híbridos 
Um sistema híbrido faz sentido quando se quer portabilidade com desempenho razoável. A compilação total faz sentido quando o desempenho máximo é prioridade e a portabilidade não.

25. Just in Time 
Rodar código intermediário com boa velocidade. compilar sob demanda os trechos usados. JIT combina portabilidade do bytecode com execução rápida das partes quentes, ao custo de um atraso inicial e maior uso de memória na compilação em tempo de execução.

26. Pré-processadores 
Entra avaliando se a linguagem depende de macros/inclusões processadas antes da compilação. pré-processadores dão flexibilidade, mas macros mal usadas reduzem legibilidade e escondem erros, o que pesa na escolha.

27. Ambientes de Programação O raciocínio ignora que ferramentas afetam produtividade, não só a linguagem em si.Mesmo com o código funcionando, um bom ambiente de programação reduz custo, erros e pode ser decisivo na escolha da linguagem.

