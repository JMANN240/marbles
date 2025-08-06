import argparse
import os

import google_auth_oauthlib
import googleapiclient.discovery
import googleapiclient.http
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials

SCOPES = ["https://www.googleapis.com/auth/youtube.upload"]
TOKEN_FILE = 'youtube_token.json'


def get_authenticated_youtube():
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

    youtube = googleapiclient.discovery.build(
        "youtube", "v3", credentials=creds)
    return youtube


def upload_video(youtube, path, title, description, tags):
    request_body = {
        "snippet": {
            "categoryId": "24",
            "title": title,
            "description": description,
            "tags": tags
        },
        "status": {
            "privacyStatus": "public",
            "selfDeclaredMadeForKids": False
        }
    }

    request = youtube.videos().insert(
        part="snippet,status",
        body=request_body,
        media_body=googleapiclient.http.MediaFileUpload(
            path, chunksize=-1, resumable=True)
    )

    response = None

    while response is None:
        status, response = request.next_chunk()
        if status:
            print(f"Upload {int(status.progress()*100)}%")

        print(f"Video uploaded with ID: {response['id']}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Upload a video to YouTube.")
    parser.add_argument("--path", required=True,
                        help="Path to the video file.")
    parser.add_argument("--title", required=True, help="Title of the video.")
    parser.add_argument("--description", required=True,
                        help="Description of the video.")
    parser.add_argument(
        "--tags", type=lambda s: [t.strip() for t in s.split(",")])

    args = parser.parse_args()

    youtube = get_authenticated_youtube()
    upload_video(youtube,
                 path=args.path,
                 title=args.title,
                 description=args.description,
                 tags=args.tags
                 )
