use std::io::{self, Write};

fn main() {
	println!("Hello World");
	print!("Digite um número: ");
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	let numero: i64 = input.trim().parse().unwrap_or(0);

	for i in 1..=10 {
		println!("{} x {} = {}", numero, i, numero * i);
	}
}
// ------------------------------------------------- VAGA ---------------------------------------------------------------------

// Software Engineer (Rust) | BTG Empresas
// São Paulo, SP (presencial)

// Sobre a área
// O time do BTG Empresas está construindo uma HAL (Hardware Abstraction Layer) própria para terminais POS (maquininhas),
// com o objetivo de acabar com a dependência de fornecedores terceiros no processamento de pagamentos.

// No dia a dia
// • Implementar o núcleo da biblioteca HAL em Rust, seguindo uma arquitetura multi-fabricante já definida
// • Desenvolver um micro-kernel de pagamentos EMV com orçamento de latência de cerca de 2 ms
// • Criar adapters que escondem as diferenças entre fabricantes de terminais (PAX, Sunmi, Positivo)
// • Construir pontes FFI entre Rust e Flutter/Dart e entre Rust e bibliotecas nativas em C/Java
// • Otimizar operações críticas para processadores embarcados ARM Cortex-A53
// • Validar o código com benchmarks e testes em terminais físicos
// • Trabalhar em pair programming com o Principal HAL Engineer e outros devs Rust

// Requisitos e Qualificações
// • Graduação em Engenharia, Ciência da Computação ou áreas relacionadas
// • Rust avançado com foco em performance crítica, ou experiência forte em C/C++ de sistemas
// • FFI e interoperabilidade entre linguagens (Rust↔C, Rust↔Dart, C↔Java/JNI)
// • Serialização eficiente (JSON, MessagePack, Protocol Buffers)
// • Programação concorrente: threads, mutexes, atomics, async
// • Sistemas embarcados, mobile ou tempo real: gestão de memória e recursos limitados
// • Cross-compilation e build systems: Cargo, Make/CMake, CI/CD para múltiplos targets
// • Debugging e profiling de aplicações de alta performance em produção
// • System design: camadas de abstração de hardware, arquiteturas de plugins, APIs multiplataforma

// Diferenciais
// • Experiência com meios de pagamento ou sistemas financeiros críticos
// • Contribuições em bibliotecas, frameworks ou drivers open source em Rust ou C/C++
// • Async/await, modelo de ownership, ecossistema Cargo, Tokio e Serde
// • Serialização zero-copy (FlatBuffers, Cap'n Proto)
// • Android NDK e integração nativa via JNI
// • Otimização com SIMD, cache e ferramentas como perf, valgrind e heaptrack

// O que oferecemos:
// • Participação nos Lucros e Resultados (PLR);
// • Vale-Alimentação e Vale-Refeição;
// • Assistência Médica;
// • Assistência Odontológica;
// • Auxílio-Creche/Babá;
// • Vale-Transporte;
// • Wellhub e TotalPass;
// • Programa de Apoio Pessoal (EAP);
// • Previdência Privada e Seguro de Vida por adesão;
// • Licença-maternidade e paternidade estendidas (Empresa Cidadã).



// ------------------------------------------------- FAIXA SALARIAL ---------------------------------------------------------------------
// Júnior: R$ 4.000 até R$ 9.000; 
// Pleno:  R$ 9.000 até R$ 20.000; 
// Sênior: R$ 18.000 até R$ 38.000.

// ------------------------------------------------- PARADIGMA ---------------------------------------------------------------------

// MULTIPARADIGMA: Procedural, Funcional, Concorrente e Genérico (traits no lugar de classes e herança).
