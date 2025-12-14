source .env;

RESPONSE=$(curl "https://graph.instagram.com/refresh_access_token?grant_type=ig_refresh_token&access_token=$INSTAGRAM_USER_ACCESS_TOKEN");
echo "$RESPONSE";
ACCESS_TOKEN=$(echo "$RESPONSE" | jq '.access_token' | perl -pe 's|^"(.*)"$|\1|g');
echo "$ACCESS_TOKEN";
perl -pi -e "s|INSTAGRAM_USER_ACCESS_TOKEN=.*$|INSTAGRAM_USER_ACCESS_TOKEN=$ACCESS_TOKEN|g" .env