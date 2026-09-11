from datetime import datetime, timezone

from fastapi.testclient import TestClient

from app.analytics import OrderItemEvent, build_product_metrics
from app.main import app, rank, RecommendationRequest

client = TestClient(app)


def test_health():
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json() == {"service": "olympus-ai", "status": "ok"}


def test_recommendations_rank_popular_products_and_respect_limit():
    result = rank(
        RecommendationRequest(
            product_ids=["p1", "p2", "p3"],
            limit=2,
            popularity={"p1": 100, "p2": 20, "p3": 1},
            price={"p1": 100, "p2": 10, "p3": 1},
        )
    )
    assert len(result) == 2
    assert result[0].product_id == "p1"
    assert all(item.score >= 0 for item in result)


def test_recommendations_are_deterministic_on_equal_scores():
    result = rank(
        RecommendationRequest(product_ids=["b", "a"], limit=2)
    )
    assert [item.product_id for item in result] == ["a", "b"]


def test_empty_recommendation_input():
    assert rank(RecommendationRequest()) == []


def test_product_metrics_aggregate_revenue_and_units():
    timestamp = datetime(2026, 1, 1, tzinfo=timezone.utc)
    events = [
        OrderItemEvent("p1", 2, 10.0, timestamp),
        OrderItemEvent("p1", 3, 5.0, timestamp),
        OrderItemEvent("p2", 1, 25.0, timestamp),
    ]
    metrics = build_product_metrics(events)
    assert metrics == [
        {"product_id": "p1", "units_sold": 5.0, "revenue": 35.0},
        {"product_id": "p2", "units_sold": 1.0, "revenue": 25.0},
    ]


def test_product_metrics_ignore_invalid_events():
    timestamp = datetime(2026, 1, 1, tzinfo=timezone.utc)
    metrics = build_product_metrics(
        [
            OrderItemEvent("valid", 2, 4.0, timestamp),
            OrderItemEvent("zero", 0, 10.0, timestamp),
            OrderItemEvent("negative", -1, 10.0, timestamp),
            OrderItemEvent("free", 2, 0.0, timestamp),
        ]
    )
    assert metrics == [
        {"product_id": "free", "units_sold": 2.0, "revenue": 0.0},
        {"product_id": "valid", "units_sold": 2.0, "revenue": 8.0},
    ]


def test_recommendation_endpoint_validation():
    response = client.post(
        "/v1/recommendations",
        json={"product_ids": ["p1"], "limit": 0},
    )
    assert response.status_code == 422


def test_analytics_endpoint():
    response = client.post(
        "/v1/analytics/product-metrics",
        json={
            "order_items": [
                {
                    "product_id": "p1",
                    "quantity": 2,
                    "unit_price": 12.5,
                    "created_at": "2026-01-01T00:00:00Z",
                }
            ]
        },
    )
    assert response.status_code == 200
    assert response.json() == [
        {"product_id": "p1", "units_sold": 2.0, "revenue": 25.0}
    ]
