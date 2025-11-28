# Landing Page

This directory contains the landing page for Sovereign.

## Deployment to GitHub Pages

1. Go to your GitHub repository: `https://github.com/lehendo/sovereign`
2. Navigate to **Settings** → **Pages**
3. Under **Source**, select **Deploy from a branch**
4. Select **main** (or **master**) branch
5. Select **/docs** folder
6. Click **Save**

Your site will be available at: `https://lehendo.github.io/sovereign/`

## Custom Domain (Optional)

If you have a custom domain (e.g., `sovereign.app`):

1. Add a `CNAME` file in the `docs/` directory with your domain name
2. Configure DNS records as per GitHub Pages documentation
3. Update the install script URL in `index.html` if needed

## Notes

- The placeholder screenshot should be replaced with an actual screenshot of the Sovereign UI
- Update release download URLs once you create your first GitHub release
- The install script URL (`https://sovereign.app/install.sh`) can be updated when you set up a custom domain

