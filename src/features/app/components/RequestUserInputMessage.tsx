import { useState } from "react";
import type {
  RequestUserInputRequest,
  RequestUserInputResponse,
} from "../../../types";
import { respondToServerRequest } from "../../../services/tauri";

interface Props {
  request: RequestUserInputRequest;
  onComplete: () => void;
}

export function RequestUserInputMessage({ request, onComplete }: Props) {
  const [answers, setAnswers] = useState<Record<string, string[]>>({});
  const [submitting, setSubmitting] = useState(false);

  const handleOptionSelect = (questionId: string, option: string) => {
    setAnswers((prev) => ({
      ...prev,
      [questionId]: [option],
    }));
  };

  const handleTextInput = (questionId: string, value: string) => {
    setAnswers((prev) => ({
      ...prev,
      [questionId]: [value],
    }));
  };

  const handleSubmit = async () => {
    if (submitting) {
      return;
    }
    setSubmitting(true);
    try {
      const response: RequestUserInputResponse = {
        answers: Object.fromEntries(
          Object.entries(answers).map(([id, answer]) => [id, { answers: answer }]),
        ),
      };
      await respondToServerRequest(
        request.workspace_id,
        request.request_id,
        response,
      );
      onComplete();
    } catch (error) {
      console.error("Failed to submit user input:", error);
    } finally {
      setSubmitting(false);
    }
  };

  const allQuestionsAnswered = request.params.questions.every(
    (question) => answers[question.id]?.length,
  );

  return (
    <div className="request-user-input">
      <div className="request-user-input-header">
        <span className="request-user-input-icon" aria-hidden>
          ❓
        </span>
        <span>Agent needs your input</span>
      </div>

      {request.params.questions.map((question) => (
        <div key={question.id} className="request-user-input-question">
          <div className="question-header">{question.header}</div>
          <div className="question-text">{question.question}</div>

          {question.options?.length ? (
            <div className="question-options">
              {question.options.map((option) => (
                <button
                  key={option.label}
                  type="button"
                  className={`option-button ${
                    answers[question.id]?.[0] === option.label ? "selected" : ""
                  }`}
                  onClick={() => handleOptionSelect(question.id, option.label)}
                >
                  <span className="option-label">{option.label}</span>
                  <span className="option-description">{option.description}</span>
                </button>
              ))}
            </div>
          ) : (
            <input
              type="text"
              className="question-input"
              placeholder="Type your answer..."
              value={answers[question.id]?.[0] ?? ""}
              onChange={(event) => handleTextInput(question.id, event.target.value)}
            />
          )}
        </div>
      ))}

      <button
        className="primary submit-button"
        disabled={!allQuestionsAnswered || submitting}
        onClick={handleSubmit}
      >
        {submitting ? "Submitting..." : "Submit Answers"}
      </button>
    </div>
  );
}
