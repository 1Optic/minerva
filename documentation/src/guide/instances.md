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

## Update Deployment Process

### Verifying Pending Changes

When an existing Minerva instance deployment needs to be updated, the Minerva
CLI can be used to apply all required changes. To first check what will be
updated, the `diff` command can be used:

```
$ minerva diff --with-dir examples/tiny_instance_v2
Differences dir('examples/tiny_instance_v1') -> dir('examples/tiny_instance_v2')
* Trend(hub_node_main_15m.outside_temp)
* AddTrends(TrendStorePart(hub_node_main_15m), 2):
 - power_kwh: numeric
 - freq_power: numeric

* RemoveTrends(TrendStorePart(hub_node_main_15m), 2):
 - bytes_tx
 - bytes_rx

* ModifyTrendDataTypes(hub_node_main_15m, 2/4):
 - outside_temp: bigint -> numeric
 - inside_temp: integer -> numeric

* RemoveTrendStorePart(t_1month)
* RemoveTrendStorePart(u_1month)
* RemoveTrendStorePart(hub_node_energy_15m)
* AddTrendStore(TrendStore(hub, v-network, 15m))
 - hub_v-network_main_15m

* ChangeAttribute(AttributeStore(hub, node), longitude)
* ChangeAttribute(AttributeStore(hub, node), latitude)
* AddAttributes(AttributeStore(hub, node), 1):
 - equipment_type: text

* AddTrendMaterialization(TrendViewMaterialization('hub_v-network_main_15m'))
```

### Applying Pending Changes

#### Manual Confirmation

When the output of the `diff` command matches with the expected changes, you can continue with the actual updating. Updating can be done using the `update` command and by default, it asks for confirmation for each change:
```
$ minerva update examples/tiny_instance_v2
Applying changes:


* [1/11] ModifyTrendDataTypes(hub_node_main_15m, 1/4):
 - outside_temp: numeric -> bigint

Apply change?:
> Yes
  No
  Show trend value information
```

Choosing 'Yes' for this change causes it to be applied immediately and a
confirmation of what has been changed is shown:
```
Apply change?: Yes
> Altered trend data types for trend store part 'hub_node_main_15m'


* [2/11] ChangeAttribute(AttributeStore(hub, node), longitude)
Apply change?:
> Yes
  No
```

Choosing 'No' for a change causes it to be skipped and a confirmation prompt of
the next change (if there is any) will be shown:
```
* [2/11] ChangeAttribute(AttributeStore(hub, node), longitude)
Apply change?: No


* [3/11] ChangeAttribute(AttributeStore(hub, node), latitude)
Apply change?:
> Yes
  No
```

#### Unattended Deployment

When you are sure that all pending changes are the correct ones, you can use
the `--non-interactive` option. This will apply all changes without prompting
for confirmation:
```
$ minerva update --non-interactive examples/tiny_instance_v2
Applying changes:


* [1/2] AddTrends(TrendStorePart(hub_node_main_15m), 1):
 - traffic_bytes: numeric

> Added 1 trends to trend store part 'hub_node_main_15m'


* [2/2] RemoveTrends(TrendStorePart(hub_node_main_15m), 1):
 - power_kwh

> Removed 1 trends from trend store part 'hub_node_main_15m'
```

### Minimizing Risk

When deploying changes on larger instances with high availability requirements,
you can minimize the risk by using a deployment scheme with safety measures. In
these cases it is advised to first only apply the additions and only delete
data at the very end.

To minimize the risk, you are advised to follow the following steps in this order:

1. Apply counter and attribute additions
2. Verify data processing for counters and attributes
3. Verify data availability for new counters and attributes
4. Rename trends and attributes to delete 
5. Look for impact after renaming counters and attributes to be removed in the Minerva instance
6. Delete counters and attributes that were renamed earlier

#### Apply Additions Only

The regular `update` command can be used with a special option `--ignore-deletions` to prevent attributes and trends being deleted:
```
$ minerva update --ignore-deletions examples/tiny_instance_v2
Applying changes:


* [1/1] AddTrends(TrendStorePart(hub_node_main_15m), 1):
 - traffic_bytes: numeric

Apply change?: Yes
> Added 1 trends to trend store part 'hub_node_main_15m'
```

**NOTE**

After applying the additions, make sure any services ingesting the data are
restarted so that they pick up the new trends and attributes.

#### Rename Trends And Attributes For Deletion

To be able to roll back deletions, there is an option to first rename the
corresponding columns to see the impact of deleting the trends and attributes
without actually deleting them (including the data).

After renaming the trends and attributes, any consumers that might break should
be checked.

#### Delete Renamed Trends And Attributes

When every consumer of the data is verified to be working, the actual deletion
of trends and attributes can be done. This will also delete any data for those
trends and attributes.
