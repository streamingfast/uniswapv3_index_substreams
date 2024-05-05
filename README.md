# UniswapV3 index Substreams 

This substreams provides a pre-filtered map module that map all `uniswapV3` events. The `map_events` is filtered through the `index_pool_events` module which index all the blocks containing data related to the `uniswapV3` protocol. Any substreams users can use this pre-filtered `map_events` as input of another module and take profit from this `uniswapV3` protocol filter (processing only blocks containing only `uniswapV3` data). 

## Modules description

```yaml
- name: index_pool_events
    kind: blockIndex
    initialBlock: 12369621
    inputs:
      - source: sf.ethereum.type.v2.Block
      - store: store_factory_pool_created
    output:
      type: proto:sf.substreams.index.v1.Keys
```

```yaml
- name: map_events
    kind: map
    initialBlock: 12369621
    inputs:
      - source: sf.ethereum.type.v2.Block
      - store: store_factory_pool_created
    blockFilter:
      module: index_pool_events
      query: uniswapv3  
    output:
      type: proto:contract.v1.Events
```