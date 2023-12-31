# Sistema_Rust 
<img src="https://github.com/nojirilucas/Hipercampo_C/assets/103136574/071f7136-9f3f-4522-9d44-e6c818a7b4e8" width="350">

Desenvolver um sistema utilizando a linguagem escolhida pela equipe. O sistema deve apresentar um menu em
linha de comando que permite a inclusão, alteração, remoção e visualização dos dados das entidades abaixo. As
classes das entidades devem conter apenas atributos e métodos que manipulam os dados. A interface em linha de
comando deve ser implementada em uma ou mais classes separadas:

1. Pessoa (abstrata): nome (String), endereço (String);
2. Cliente (subclasse de Pessoa): rg (String), data de nascimento (Date);
3. Produto: código (int), nome (String), valor (float);
4. Totalizavel (abstrata ou interface): define um método abstrato chamado total que retorna o valor total
(float);
5. Venda (subclasse de Totalizavel): número (int), data (Date), cliente (Cliente), itens (lista ou array de
ItemVenda).
O método total deve calcular a soma dos totais de cada item.

• ItemVenda (subclasse de Totalizavel): produto (Produto), valor (float), quantidade (int).
O valor é copiado do valor do produto no momento da venda, assim, mesmo que o valor do produto mude posteriormente,
as vendas mantêm o valor do momento em que foram realizadas).
O método total deve calcular o valor vezes a quantidade.

![menu](https://github.com/nojirilucas/Sistema_Rust/assets/103136574/60a6ffc9-fe43-4ee4-ac35-716a65203000)
