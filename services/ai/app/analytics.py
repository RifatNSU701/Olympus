from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Iterable


@dataclass(frozen=True)
class OrderItemEvent:
    product_id: str
    quantity: int
    unit_price: float
    created_at: datetime


def build_product_metrics(events: Iterable[OrderItemEvent]) -> list[dict[str, float | str]]:
    metrics: dict[str, dict[str, float]] = {}
    for event in events:
        if event.quantity <= 0 or event.unit_price < 0:
            continue
        row = metrics.setdefault(event.product_id, {"units_sold": 0.0, "revenue": 0.0})
        row["units_sold"] += event.quantity
        row["revenue"] += event.quantity * event.unit_price
    return [
        {"product_id": product_id, "units_sold": values["units_sold"], "revenue": round(values["revenue"], 2)}
        for product_id, values in sorted(metrics.items())
    ]
