use super::base_styles::BASE_STYLES;

pub fn get_template() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sign Up - NanoLink</title>
    <style>
        BASE_STYLES_PLACEHOLDER

        body {
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .auth-container {
            background-color: #1A1A1D;
            padding: 2rem;
            border-radius: 5px;
            border: 1px solid #222831;
            width: 100%;
            max-width: 400px;
        }

        .logo {
            text-align: center;
            margin-bottom: 2rem;
        }

        .logo h1 {
            font-size: 1.5rem;
            font-weight: 600;
            color: #F2613F;
            margin-bottom: 0.25rem;
        }

        .logo p {
            color: #a1a1aa;
            font-size: 0.875rem;
        }

        .btn {
            width: 100%;
            padding: 0.75rem;
            border: 1px solid #9B3922;
            border-radius: 5px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
            font-size: 0.875rem;
            margin-bottom: 1rem;
            background-color: #F2613F;
            color: white;
            justify-content: center;
        }

        .btn:hover {
            background-color: #ff6f4b;
            border-color: #9B3922;
        }

        .btn:disabled {
            background-color: #3f3f46;
            border-color: #3f3f46;
            cursor: not-allowed;
        }

        .auth-links {
            text-align: center;
            margin-top: 1.5rem;
        }

        .auth-links a {
            color: #F2613F;
            text-decoration: none;
            font-weight: 500;
            font-size: 0.875rem;
        }

        .auth-links a:hover {
            text-decoration: underline;
        }

        .back-link {
            text-align: center;
            margin-bottom: 2rem;
        }

        .back-link a {
            color: #a1a1aa;
            text-decoration: none;
            font-size: 0.875rem;
            display: inline-flex;
            align-items: center;
            gap: 0.25rem;
        }

        .back-link a:hover {
            color: #e4e4e7;
        }

        .error-message {
            background-color: rgba(239, 68, 68, 0.1);
            border: 1px solid #ef4444;
            color: #ef4444;
            padding: 0.75rem;
            border-radius: 5px;
            margin-bottom: 1rem;
            text-align: center;
            font-size: 0.875rem;
        }

        .success-message {
            background-color: rgba(34, 197, 94, 0.1);
            border: 1px solid #22c55e;
            color: #22c55e;
            padding: 0.75rem;
            border-radius: 5px;
            margin-bottom: 1rem;
            text-align: center;
            font-size: 0.875rem;
        }
    </style>
</head>
<body style="padding: 1rem;">
    <div class="auth-container">
        <div class="back-link">
            <a href="/">
                <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M19 12H5"/>
                    <path d="M12 19l-7-7 7-7"/>
                </svg>
                Back to Home
            </a>
        </div>

        <div style="display: flex; flex-direction: column; align-items: center; padding-bottom: 2.5rem; gap: 0.75rem;">
            <a href="/" style="text-decoration: none; display: flex; align-items: center; gap: 0.5rem; font-weight: 600; font-size: 1.5rem; color: #F2613F;">
                <svg style="width: 1.5rem; height: 1.5rem;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.042 21.672 13.684 16.6m0 0-2.51 2.225.569-9.47 5.227 7.917-3.286-.672Zm-7.518-.267A8.25 8.25 0 1 1 20.25 10.5M8.288 14.212A5.25 5.25 0 1 1 17.25 10.5" />
                </svg>
                NanoLink
            </a>
            <p>Create your account</p>
        </div>

        <div id="errorMessage" class="error-message hidden"></div>
        <div id="successMessage" class="success-message hidden"></div>

        <form id="signupForm">
            <div class="form-group">
                <label for="username" style="display: flex; align-items: center; gap: 0.25rem;">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.25rem;">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                        <circle cx="12" cy="7" r="4"/>
                    </svg>
                    Username
                </label>
                <input type="text" id="username" class="form-input" required>
            </div>
            
            <div class="form-group">
                <label for="email" style="display: flex; align-items: center; gap: 0.25rem;">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.25rem;">
                        <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
                        <polyline points="22,6 12,13 2,6"/>
                    </svg>
                    Email
                </label>
                <input type="email" id="email" class="form-input" required>
            </div>
            
            <div class="form-group">
                <label for="password" style="display: flex; align-items: center; gap: 0.25rem;">
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="display: inline; margin-right: 0.25rem;">
                        <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                        <circle cx="12" cy="16" r="1"/>
                        <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                    </svg>
                    Password
                </label>
                <input type="password" id="password" class="form-input" required>
            </div>
            
            <button type="submit" class="btn" id="submitBtn">
                <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
                    <circle cx="9" cy="7" r="4"/>
                    <line x1="19" y1="8" x2="19" y2="14"/>
                    <line x1="22" y1="11" x2="16" y2="11"/>
                </svg>
                Sign Up
            </button>
        </form>

        <div class="auth-links">
            <p>Already have an account? <a href="/signin">Sign in here</a></p>
        </div>
    </div>

    <script>
        document.getElementById('signupForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const submitBtn = document.getElementById('submitBtn');
            const errorMessage = document.getElementById('errorMessage');
            const successMessage = document.getElementById('successMessage');
            const username = document.getElementById('username').value;
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;
            
            submitBtn.disabled = true;
            submitBtn.textContent = 'Creating Account...';
            errorMessage.classList.add('hidden');
            successMessage.classList.add('hidden');
            
            try {
                const response = await fetch('/register', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ username, email, password })
                });
                
                const data = await response.json();
                
                if (response.ok) {
                    successMessage.textContent = 'Account created successfully! Redirecting to sign in...';
                    successMessage.classList.remove('hidden');
                    
                    setTimeout(() => {
                        window.location.href = '/signin';
                    }, 2000);
                } else {
                    errorMessage.textContent = data.error;
                    errorMessage.classList.remove('hidden');
                }
            } catch (error) {
                errorMessage.textContent = 'Network error. Please try again.';
                errorMessage.classList.remove('hidden');
            } finally {
                submitBtn.disabled = false;
                submitBtn.innerHTML = `
                    <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
                        <circle cx="9" cy="7" r="4"/>
                        <line x1="19" y1="8" x2="19" y2="14"/>
                        <line x1="22" y1="11" x2="16" y2="11"/>
                    </svg>
                    Sign Up
                `;
            }
        });

        if (localStorage.getItem('authToken')) {
            window.location.href = '/';
        }
    </script>
</body>
</html>
"#.replace("BASE_STYLES_PLACEHOLDER", BASE_STYLES)
}