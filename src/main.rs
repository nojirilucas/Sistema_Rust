use chrono::prelude::*;

struct Pessoa {
    nome: String,
    endereco: String,
}

impl Pessoa {
    fn new(nome: String, endereco: String) -> Self {
        Pessoa { nome, endereco }
    }
}

struct Cliente {
    pessoa: Pessoa,
    rg: String,
    data_nascimento: NaiveDate,
    produtos: Vec<Produto>,
}

impl Cliente {
    fn new(nome: String, endereco: String, rg: String, data_nascimento: NaiveDate) -> Self {
        let pessoa = Pessoa::new(nome, endereco);
        Cliente {
            pessoa,
            rg,
            data_nascimento,
            produtos: Vec::new(),
        }
    }

    fn atualizar_rg(&mut self, novo_rg: String) {
        self.rg = novo_rg;
    }

    fn atualizar_data_nascimento(&mut self, nova_data: NaiveDate) {
        self.data_nascimento = nova_data;
    }

    fn atualizar_nome(&mut self, novo_nome: String) {
        self.pessoa.nome = novo_nome;
    }

    fn atualizar_endereco(&mut self, novo_endereco: String) {
        self.pessoa.endereco = novo_endereco;
    }

    fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.push(produto);
    }

    fn valor_total_produtos(&self) -> f32 {
        self.produtos.iter().map(|produto| produto.valor).sum()
    }
}

struct Produto {
    codigo: i32,
    nome: String,
    valor: f32,
}

impl Produto {
    fn new(codigo: i32, nome: String, valor: f32) -> Self {
        Produto {
            codigo,
            nome,
            valor,
        }
    }
}

trait Totalizavel {
    fn total(&self) -> f32;
}

struct ItemVenda {
    produto: Produto,
    valor: f32,
    quantidade: i32,
}

impl ItemVenda {
    fn new(produto: Produto, quantidade: i32) -> Self {
        let valor = produto.valor;
        ItemVenda {
            produto,
            valor,
            quantidade,
        }
    }
}

impl Totalizavel for ItemVenda {
    fn total(&self) -> f32 {
        self.valor * self.quantidade as f32
    }
}

struct Venda {
    numero: i32,
    data: NaiveDateTime,
    clientes: Vec<Cliente>,
    itens: Vec<ItemVenda>,
}

impl Venda {
    fn new(numero: i32) -> Self {
        let data = Local::now().naive_local(); // Obtém a data e hora atual
        Venda {
            numero,
            data,
            clientes: Vec::new(),
            itens: Vec::new(),
        }
    }

    fn adicionar_item(&mut self, item: ItemVenda) {
        self.itens.push(item);
    }

    fn adicionar_cliente(&mut self, cliente: Cliente) {
        self.clientes.push(cliente);
    }

    fn valor_total_produtos(&self) -> f32 {
        self.clientes
            .iter()
            .map(|cliente| cliente.valor_total_produtos())
            .sum()
    }
}

impl Totalizavel for Venda {
    fn total(&self) -> f32 {
        self.itens.iter().map(|item| item.total()).sum()
    }
}

fn exibir_menu() {
    println!("========== Menu do Cliente ===========");
    println!("1. Exibir clientes");
    println!("2. Atualizar cliente");
    println!("3. Remover cliente");
    println!("4. Cadastrar cliente");
    println!("5. Voltar");
    println!("======================================\n");
    println!("Escolha uma opção: ");
}

fn exibir_informacoes_cliente(cliente: &Cliente) {
    println!("Nome: {}", cliente.pessoa.nome);
    println!("Endereço: {}", cliente.pessoa.endereco);
    println!("RG: {}", cliente.rg);
    println!("Data de nascimento: {}\n", cliente.data_nascimento);
    println!("Produtos:");
    for produto in &cliente.produtos {
        println!("Código: {}", produto.codigo);
        println!("Nome: {}", produto.nome);
        println!("Valor: {}\n", produto.valor);
    }
    println!("Valor total dos produtos: R$ {:.2}\n", cliente.valor_total_produtos());
}

fn atualizar_informacoes_cliente(cliente: &mut Cliente) {
    println!("Digite o novo nome: ");
    let novo_nome = read_line();
    cliente.atualizar_nome(novo_nome);

    println!("Digite o novo endereço: ");
    let novo_endereco = read_line();
    cliente.atualizar_endereco(novo_endereco);

    println!("Digite o novo RG: ");
    let novo_rg = read_line();
    cliente.atualizar_rg(novo_rg);

    println!("Digite a nova data de nascimento (no formato AAAA-MM-DD): ");
    let nova_data_str = read_line();
    let nova_data = NaiveDate::parse_from_str(&nova_data_str, "%Y-%m-%d")
        .expect("Data de nascimento inválida\n");
    cliente.atualizar_data_nascimento(nova_data);

    println!("As informações do cliente foram atualizadas com sucesso!\n");
}

fn remover_cliente(venda: &mut Venda) {
    println!("Digite o número do cliente a ser removido: ");
    let indice_str = read_line();
    let indice = indice_str.parse::<usize>().unwrap_or(0);

    if indice < venda.clientes.len() {
        venda.clientes.remove(indice);
        println!("Cliente removido com sucesso!\n");
    } else {
        println!("Índice inválido. Nenhum cliente foi removido.\n");
    }
}

fn cadastrar_cliente(venda: &mut Venda) {
    println!("Digite o nome do cliente: ");
    let nome = read_line();

    println!("Digite o endereço do cliente: ");
    let endereco = read_line();

    println!("Digite o RG do cliente: ");
    let rg = read_line();

    println!("Digite a data de nascimento do cliente (no formato AAAA-MM-DD): ");
    let data_nascimento_str = read_line();
    let data_nascimento = NaiveDate::parse_from_str(&data_nascimento_str, "%Y-%m-%d")
        .expect("Data de nascimento inválida\n");

    let mut novo_cliente = Cliente::new(nome, endereco, rg, data_nascimento);

    loop {
        println!("Deseja adicionar um produto? (S/N)");
        let resposta = read_line().to_lowercase();

        if resposta == "n" {
            break;
        } else if resposta != "s" {
            println!("Resposta inválida. Digite 'S' para sim ou 'N' para não.");
            continue;
        }

        println!("Digite o código do produto: ");
        let codigo_produto = read_line().parse::<i32>().unwrap_or(0);

        println!("Digite o nome do produto: ");
        let nome_produto = read_line();

        println!("Digite o valor do produto: ");
        let valor_produto = read_line().parse::<f32>().unwrap_or(0.0);

        let novo_produto = Produto::new(codigo_produto, nome_produto, valor_produto);
        novo_cliente.adicionar_produto(novo_produto);
    }

    venda.adicionar_cliente(novo_cliente);
    println!("Cliente cadastrado com sucesso!\n");
}

fn exibir_clientes(venda: &Venda) {
    if venda.clientes.is_empty() {
        println!("Nenhum cliente cadastrado.\n");
    } else {
        for (indice, cliente) in venda.clientes.iter().enumerate() {
            println!("Índice: {}", indice);
            exibir_informacoes_cliente(cliente);
        }
    }
}

fn main() {
    let mut venda = Venda::new(1);

    loop {
        exibir_menu();

        let opcao = read_line().parse::<i32>().unwrap_or(0);

        match opcao {
            1 => {
                exibir_clientes(&venda);
            }
            2 => {
                println!("Digite o número do cliente a ser atualizado: ");
                let indice_str = read_line();
                let indice = indice_str.parse::<usize>().unwrap_or(0);

                if indice < venda.clientes.len() {
                    let cliente = &mut venda.clientes[indice];
                    atualizar_informacoes_cliente(cliente);
                } else {
                    println!("Índice inválido.\n");
                }
            }
            3 => {
                remover_cliente(&mut venda);
            }
            4 => {
                cadastrar_cliente(&mut venda);
            }
            5 => {
                break;
            }
            _ => {
                println!("Opção inválida. Tente novamente.\n");
            }
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
