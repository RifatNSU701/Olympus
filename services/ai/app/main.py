from fastapi import FastAPI
from pydantic import BaseModel, Field

app = FastAPI(title="Olympus AI", version="0.1.0")

class RecommendationRequest(BaseModel):
    product_ids: list[str] = Field(default_factory=list)
    limit: int = Field(default=5, ge=1, le=20)

class Recommendation(BaseModel):
    product_id: str
    score: float
    reason: str

@app.get("/health")
def health() -> dict[str, str]:
    return {"service": "olympus-ai", "status": "ok"}

@app.post("/v1/recommendations", response_model=list[Recommendation])
def recommendations(request: RecommendationRequest) -> list[Recommendation]:
    # Foundation endpoint; ranking will be replaced by a trained model.
    return [
        Recommendation(product_id=product_id, score=1.0 - (index * 0.05), reason="candidate")
        for index, product_id in enumerate(request.product_ids[: request.limit])
    ]
