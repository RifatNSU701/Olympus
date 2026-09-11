import { FormEvent, useEffect, useState } from 'react';
import { ArrowRight, LogOut, UserRound, X } from 'lucide-react';

const API = import.meta.env.VITE_API_URL ?? 'http://localhost:8080';

type User = { id: string; email: string; role: string; status: string; full_name: string };

type Props = { open: boolean; onClose: () => void; onAuthenticated: (user: User) => void };

export function AuthDialog({ open, onClose, onAuthenticated }: Props) {
  const [mode, setMode] = useState<'login' | 'register'>('login');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [fullName, setFullName] = useState('');
  const [message, setMessage] = useState('');
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (!open) return;
    setMessage('');
    setPassword('');
  }, [open, mode]);

  if (!open) return null;

  async function submit(event: FormEvent) {
    event.preventDefault();
    setMessage('');
    setBusy(true);
    try {
      const endpoint = mode === 'login' ? '/api/v1/auth/login' : '/api/v1/auth/register';
      const body = mode === 'login' ? { email, password } : { email, password, full_name: fullName };
      const response = await fetch(`${API}${endpoint}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
        body: JSON.stringify(body),
      });
      if (!response.ok) {
        const errors: Record<number, string> = {
          400: 'Please check the information you entered.',
          401: 'Invalid email or password.',
          403: 'This account is not active.',
          409: 'An account with this email already exists.',
        };
        throw new Error(errors[response.status] ?? 'Authentication failed.');
      }
      const data = await response.json();
      localStorage.setItem('olympus_token', data.access_token);
      const me = await fetch(`${API}/api/v1/auth/me`, { headers: { Authorization: `Bearer ${data.access_token}` } });
      if (!me.ok) throw new Error('Account session could not be established.');
      const user = await me.json() as User;
      onAuthenticated(user);
      onClose();
    } catch (error) {
      setMessage(error instanceof Error ? error.message : 'Something went wrong.');
    } finally {
      setBusy(false);
    }
  }

  return <div className="auth-backdrop" onClick={onClose}>
    <section className="auth-dialog" onClick={event => event.stopPropagation()} role="dialog" aria-modal="true" aria-labelledby="auth-title">
      <button className="drawer-close" aria-label="Close" onClick={onClose}><X size={20}/></button>
      <span className="eyebrow">OLYMPUS ACCOUNT</span>
      <h2 id="auth-title">{mode === 'login' ? 'Welcome back.' : 'Join Olympus.'}</h2>
      <p className="auth-copy">{mode === 'login' ? 'Sign in to manage your cart and orders.' : 'Create an account and start shopping.'}</p>
      <form onSubmit={submit}>
        {mode === 'register' && <label>Full name<input value={fullName} onChange={event => setFullName(event.target.value)} maxLength={160} autoComplete="name" required /></label>}
        <label>Email<input type="email" value={email} onChange={event => setEmail(event.target.value)} maxLength={254} autoComplete="email" required /></label>
        <label>Password<input type="password" value={password} onChange={event => setPassword(event.target.value)} maxLength={128} minLength={mode === 'register' ? 8 : 1} autoComplete={mode === 'login' ? 'current-password' : 'new-password'} required /></label>
        {message && <p className="auth-message" role="alert">{message}</p>}
        <button className="checkout-button auth-submit" disabled={busy}>{busy ? 'Please wait…' : mode === 'login' ? 'Sign in' : 'Create account'} <ArrowRight size={17}/></button>
      </form>
      <button className="auth-switch" onClick={() => setMode(mode === 'login' ? 'register' : 'login')}>{mode === 'login' ? 'Need an account? Create one' : 'Already have an account? Sign in'}</button>
    </section>
  </div>;
}

export function AccountButton({ user, onClick, onLogout }: { user: User | null; onClick: () => void; onLogout: () => void }) {
  if (!user) return <button className="account-button" onClick={onClick}><UserRound size={17}/> Sign in</button>;
  return <button className="account-button" onClick={onLogout} title="Sign out"><UserRound size={17}/> {user.full_name.split(' ')[0]} <LogOut size={14}/></button>;
}
