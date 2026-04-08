import os

import google_auth_oauthlib
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials

SCOPES = ["https://www.googleapis.com/auth/youtube.upload"]
TOKEN_FILE = 'youtube_token.json'

os.environ["OAUTHLIB_INSECURE_TRANSPORT"] = "1"
creds = None

# Load previously saved credentials if they exist
if os.path.exists(TOKEN_FILE):
	creds = Credentials.from_authorized_user_file(TOKEN_FILE, SCOPES)

# If there are no valid credentials, ask user to log in
if not creds or not creds.valid:
	if creds and creds.expired and creds.refresh_token:
		creds.refresh(Request())
	else:
		flow = google_auth_oauthlib.flow.InstalledAppFlow.from_client_secrets_file(
			"youtube_client_secrets.json", SCOPES)
		creds = flow.run_local_server()

	# Save the credentials for the next run
	with open(TOKEN_FILE, "w") as token:
		token.write(creds.to_json())
