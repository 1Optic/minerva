create table attribute."t_hub_node" as
select * from attribute."hub_node";

drop view attribute."hub_node";

alter table attribute."t_hub_node" rename to "hub_node";

select create_reference_table('attribute."hub_node"');
