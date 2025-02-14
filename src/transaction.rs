use crate::Hashable;
use std::collections::HashSet;
use super::*;

#[derive(Debug)]
pub struct Output {
    pub to_address: Address,
    pub value: u64
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_address.as_bytes()); //String como argumento, não precisa de &
        bytes.extend(&self.value.to_le_bytes());

        bytes
    }
}

#[derive(Debug)]
pub struct Transaction { //inputs se referem aos outputs de outras transações e que podem ser gastos. Outputs são valores que são enviados.
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs.iter().map(|input| input.hash()).collect()
    }
    
    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs.iter().map(|output| output.hash()).collect()
    }
    //itera sobre cada item de Vec<Output>, aplica map, "transformando" cada item em seu valor e, depois, somando tudo

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
 }



impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(self.inputs
                                .iter()
                                .flat_map(|input| input.bytes())
        );
        //.(itera sobre cada input)
        //.(aplica flat map, desembrulhando qualquer coisa que esteja aninhada dentro do input)
        //.()
        bytes.extend(self.outputs
                                .iter()
                                .flat_map(|output| output.bytes())
        );   
        
        bytes
    }
}