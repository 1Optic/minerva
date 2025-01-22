CREATE VIEW trend.power_report AS
SELECT date(timestamp), max(power_kwh) AS max_power_kwh FROM trend.hub_node_main_15m GROUP BY date(timestamp);