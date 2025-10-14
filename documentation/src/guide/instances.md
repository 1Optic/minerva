# Instances

A deployment of a Minerva database, tailored for a specific purpose, is called
a 'Minerva instance'. In a Minerva instance, you typically have a combination
of multiple entity types, corresponding attribute stores, trend stores and
notification stores.

## Instance Definition

A Minerva instance definition is a [git](https://git-scm.com/)-versioned,
standardized set of files that define all aspects of what a specific Minerva
instance deployment looks like. Example of a Minerva instance definition:

```
 📁 basic_instance
 ├─ 📄 config.json
 ├─ 📁 entity-type
 │  └─ 📄 v-equipment-cluster.yaml
 ├─ 📁 trend
 │  ├─ 📄 hub_v-network_15m.yaml
 │  ├─ 📄 hub_node_15m.yaml
 │  ├─ 📄 hub-kpi_node_15m.yaml
 ├─ 📁 notification
 │  └─ 📄 trigger-notification.yaml
 └─ 📁 attribute
    └─ 📄 hub_node.yaml
```

## Generate From Equipment Vendor Documentation

For Minerva instances that are designed to take in measurement, configuration
and notification data from technical equipment, the instance definition is often
derived from the equipment vendor documentation. Custom scripts are created to
convert the vendor's documentation to Minerva interoperable file format.

There are interoperable JSON-based file formats for
[trends](../design/trends-schema.md) and
[attributes](../design/attributes-schema.md). The scripts should generate them
and place them both in a directory:
```
 📁 extracted_definitions
 ├─ 📄 trends.json
 └─ 📄 attributes.json
```

To generate the Minerva instance definition from those trend and attribute
definitions, you use the `minerva define` command specifying the instance root
directory (`instance_a` in this example) and the directory with the trends and
attributes definition files (`extracted_definitions` in this example):

```
$ minerva define instance_a extracted_definitions
```

```
$ minerva define . extracted-definitions
Saving trend store 'TrendStore(network-5g, NRCellCU, 5m)'
Saving trend store 'TrendStore(network-5g, GNBCUUPFunction, 5m)'
Saving trend store 'TrendStore(network-5g, NRCellDU, 5m)'
Saving attribute store 'AttributeStore(network-5g, NRCellDU)'
```
