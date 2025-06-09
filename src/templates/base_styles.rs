pub const BASE_STYLES: &str = r#"
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: #0C0C0C;
            color: #F0ECE5;
            line-height: 1.6;
            min-height: 100vh;
        }

        .navbar {
            background-color: #1A1A1D;
            border-bottom: 1px solid #222831;
            padding: 1rem 0;
            position: sticky;
            top: 0;
            z-index: 100;
            backdrop-filter: blur(8px);
        }

        .nav-container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .logo {
            font-size: 1.5rem;
            font-weight: 600;
            color: #F2613F;
            text-decoration: none;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .nav-buttons {
            display: flex;
            gap: 0.75rem;
        }

        .btn {
            padding: 0.5rem 1rem;
            border: 1px solid #222831;
            border-radius: 5px;
            font-weight: 500;
            text-decoration: none;
            cursor: pointer;
            transition: all 0.2s ease;
            font-size: 0.875rem;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            background-color: transparent;
        }

        .btn-primary {
            background-color: #F2613F;
            color: white;
            border-color: #9B3922;
        }

        .btn-primary:hover {
            background-color: #ff6f4b;
            border-color: #9B3922;
        }

        .btn-secondary {
            color: #F0ECE5;
            border-color: #222831;
        }

        .btn-secondary:hover {
            background-color: #0c0c0c;
        }

        .profile-btn {
            color: #F0ECE5;
            border-color: #222831;
        }

        .profile-btn:hover {
            background-color: #0c0c0c;
        }

        .form-group {
            margin-bottom: 1.25rem;
        }

        .form-group label {
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 500;
            color: #F0ECE5;
            font-size: 0.875rem;
        }

        .form-input {
            width: 100%;
            padding: 0.75rem;
            border: 1px solid #222831;
            border-radius: 5px;
            background-color: #0C0C0C;
            color: #F0ECE5;
            font-size: 0.875rem;
            transition: border-color 0.2s ease;
        }

        .form-input:focus {
            outline: none;
            border-color: #F2613F;
        }

        .form-input::placeholder {
            color: #413F42;
        }

        .hidden {
            display: none;
        }

        .icon {
            width: 1rem;
            height: 1rem;
        }

        .icon-sm {
            width: 0.875rem;
            height: 0.875rem;
        }

        .icon-lg {
            width: 1.25rem;
            height: 1.25rem;
        }

        @media (max-width: 768px) {
            .nav-container {
                padding: 0 1rem;
            }
            
            .nav-buttons {
                gap: 0.5rem;
            }
            
            .btn {
                padding: 0.5rem 0.75rem;
                font-size: 0.8rem;
            }
        }
"#;
