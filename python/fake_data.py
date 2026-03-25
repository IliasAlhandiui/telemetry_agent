import random
from faker import Faker
from collections import Counter
from pathlib import Path

SAMPLE_COUNT = 100_000
OUTPUT_FILE = Path("data") / "sample_logs.txt"
# Weight the levels so INFO is most common, CRITICAL is rare
levels = (
    ["INFO"] * 50 + ["DEBUG"] * 20 + ["WARN"] * 15 + ["ERROR"] * 10 + ["CRITICAL"] * 5
)
print(f"level weights:{Counter(levels)}")
fake = Faker()

with open(OUTPUT_FILE, "w") as f:
    for _ in range(SAMPLE_COUNT):
        timestamp = fake.date_time_this_month().isoformat()
        level = random.choice(levels)
        ip = fake.ipv4()

        # Randomly inject PII (Personally Identifiable Information)
        chance = random.random()
        if chance < 0.10:  # 10% chance of an email
            msg = f"Failed authentication attempt for user: {fake.email()}"
        elif chance < 0.15:  # 5% chance of a credit card
            msg = f"Processing transaction for card: {fake.credit_card_number()}"
        elif level in ["ERROR", "CRITICAL"]:
            msg = f"Exception in module {fake.file_name()}: Connection refused"
        else:
            msg = (
                f"Successfully served {fake.uri_path()} in {random.randint(10, 500)}ms"
            )

        # Format: [TIMESTAMP] LEVEL IP - MESSAGE
        log_line = f"[{timestamp}] {level} {ip} - {msg}\n"
        f.write(log_line)

print(f"Created {OUTPUT_FILE}!")
