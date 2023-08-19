# Contrato Inteligente de Staking

Este es un Contrato Inteligente de Staking escrito en Rust utilizando el marco de trabajo Ink! para blockchains basados en Substrate, como Polkadot y Kusama.

## Descripción

Este contrato permite a los usuarios hacer staking (bloquear) y retirar tokens. 

## Características

- Staking: Los usuarios pueden hacer staking de tokens en el contrato.
- Retirada: Los usuarios pueden retirar tokens del contrato.
- Lectura de Era: Los usuarios pueden leer la era actual.
- Recuperación de la Cantidad Staked: Los usuarios pueden recuperar la cantidad de tokens staked por una cuenta.
- Recuperación de la Cantidad Staked en el Contrato: Los usuarios pueden recuperar la cantidad total de tokens staked en el contrato.
- Retirada de Tokens no Bloqueados: Los usuarios pueden retirar tokens no bloqueados.

## Eventos

- `Staked`: Emitido cuando un usuario hace staking de tokens. Incluye la ID de la cuenta, la era y la cantidad staked.
- `Unstaked`: Emitido cuando un usuario retira tokens. Incluye la ID de la cuenta, la era y la cantidad retirada.

## Errores

- `TransferError`: Se produjo un error durante una transferencia de tokens.
- `AddOverFlow`: Se produjo un error debido a un desbordamiento en una suma.
- `SubOverFlow`: Se produjo un error debido a un desbordamiento en una resta.
- `DSError`: Se produjo un error en el módulo subyacente DappsStaking.

## Funciones

- `bond_and_stake`: Permite a un usuario hacer staking de tokens.
- `unbond_and_unstake`: Permite a un usuario retirar tokens.
- `withdraw_unbonded`: Permite a un usuario retirar tokens no bloqueados.
- `read_current_era`: Lee la era actual.
- `get_staked_amount`: Recupera la cantidad staked para una cuenta dada.
- `read_staked_amount_on_contract`: Lee la cantidad staked en este contrato por este contrato.
- `read_contract_stake`: Lee la cantidad total staked en este contrato.

## Uso

1. Despliega el contrato en una blockchain basada en Substrate(Shibuya de Astar).
2. Los usuarios pueden interactuar con el contrato utilizando las funciones proporcionadas.
3. Se emitirán eventos cuando ocurran acciones de staking y retirada.




