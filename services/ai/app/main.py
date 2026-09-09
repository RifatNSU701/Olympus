from math import log1p

from fastapi import FastAPI
from pydantic import BaseModel, Field

app = FastAPI(title="Olympus AI", version="0.2.0")

class RecommendationRequest(BaseModel):
    product_ids: list[str] = Field(default_factory=list)
    limit: int = Field(default=5, ge=1, le=20)
    popularity: dict[str, float] = Field(default_factory=dict)
    price: dict[str, float] = Field(default_factory=dict)

class Recommendation(BaseModel):
    product_id: str
    score: float
    reason: str

def rank(request: RecommendationRequest) -> list[Recommendation]:
    if not request.product_ids:
        return []
    max_pop = max((max(0.0, value) for value in request.popularity.values()), default=1.0)
    max_price = max((max(0.0, value) for value in request.price.values()), default=1.0)
    scored = []
    for product_id in request.product_ids:
        pop = max(0.0, request.popularity.get(product_id, 0.0))
        price = max(0.0, request.price.get(product_id, 0.0))
        popularity_score = log1p(pop) / log1p(max_pop) if max_pop else 0.0
        value_score = 1.0 - (price / max_price) if max_price else 0.0
        score = round((popularity_score * 0.8) + (value_score * 0.2), 6)
        reason = "popular" if popularity_score >= value_score else "value"
        scored.append(Recommendation(product_id=product_id, score=score, reason=reason))
    return sorted(scored, key=lambda item: (-item.score, item.product_id))[: request.limit]

@app.get("/health")
def health() -> dict[str, str]:
    return {"service": "olympus-ai", "status": "ok"}

@app.post("/v1/recommendations", response_model=list[Recommendation])
def recommendations(request: RecommendationRequest) -> list[Recommendation]:
    return rank(request)
