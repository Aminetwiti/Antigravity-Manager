import { useState, useEffect } from 'react';
import { createPortal } from 'react-dom';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-opener';
import { Globe, Key, Loader2, CheckCircle2, XCircle, Info, Bot, LogIn } from 'lucide-react';
import { useAccountStore } from '../../stores/useAccountStore';
import { useTranslation } from 'react-i18next';

interface AddOpenAIAccountDialogProps {
    isOpen: boolean;
    onClose: () => void;
}

type TabResponse = 'web' | 'api';
type Status = 'idle' | 'loading' | 'success' | 'error';

export default function AddOpenAIAccountDialog({ isOpen, onClose }: AddOpenAIAccountDialogProps) {
    const { t } = useTranslation();
    const [activeTab, setActiveTab] = useState<TabResponse>('web');
    
    // Form State
    const [email, setEmail] = useState('');
    const [accessToken, setAccessToken] = useState('');
    const [sessionToken, setSessionToken] = useState('');
    const [apiKey, setApiKey] = useState('');
    
    // UI State
    const [status, setStatus] = useState<Status>('idle');
    const [message, setMessage] = useState('');
    
    const { addOpenAIWebAccount, addOpenAIAPIAccount } = useAccountStore();
    
    // Reset state when closing/opening
    useEffect(() => {
        if (isOpen) {
            resetState();
        }
    }, [isOpen]);

    const resetState = () => {
        setStatus('idle');
        setMessage('');
        setEmail('');
        setAccessToken('');
        setSessionToken('');
        setApiKey('');
    };

    const StatusAlert = () => {
        if (status === 'idle' || !message) return null;

        const styles = {
            loading: 'alert-info',
            success: 'alert-success',
            error: 'alert-error'
        };

        const icons = {
            loading: <Loader2 className="w-5 h-5 animate-spin" />,
            success: <CheckCircle2 className="w-5 h-5" />,
            error: <XCircle className="w-5 h-5" />
        };

        return (
            <div className={`alert ${styles[status]} mb-4 text-sm py-2 shadow-sm`}>
                {icons[status]}
                <span>{message}</span>
            </div>
        );
    };

    const handleSubmit = async () => {
        if (!email.trim()) {
            setStatus('error');
            setMessage(t('accounts.add.openai.error_email'));
            return;
        }

        setStatus('loading');
        setMessage(t('common.loading'));

        try {
            if (activeTab === 'web') {
                if (!accessToken.trim()) {
                    throw new Error(t('accounts.add.openai.error_token'));
                }
                await addOpenAIWebAccount(email, accessToken, sessionToken || "");
            } else {
                if (!apiKey.trim()) {
                    throw new Error(t('accounts.add.openai.error_api_key'));
                }
                await addOpenAIAPIAccount(email, apiKey);
            }
            
            setStatus('success');
            setMessage(t('common.create_success'));
            
            setTimeout(() => {
                onClose();
            }, 1500);
        } catch (error) {
            setStatus('error');
            setMessage(String(error));
        }
    };

    const handleOAuthLogin = async () => {
        try {
            setStatus('loading');
            setMessage('Generating Auth URL...');
            
            // Listen for success event
            const unlisten = await listen<any>('openai-oauth-success', (event) => {
                const { email: authEmail, access_token, refresh_token } = event.payload;
                setEmail(authEmail);
                setAccessToken(access_token);
                setSessionToken(refresh_token); // Hack: Saving refresh token in session token field for now
                setStatus('success');
                setMessage('Login successful! Credentials auto-filled.');
                
                // Clear success message after delay but keep form filled
                setTimeout(() => {
                    setStatus('idle');
                    setMessage('');
                }, 2000);
                
                unlisten();
            });

            // Start flow
            const url = await invoke<string>('start_openai_oauth_flow');
            setMessage('Opening browser for login...');
            await open(url);
            
        } catch (error) {
            setStatus('error');
            setMessage(`OAuth Error: ${error}`);
        }
    };

    if (!isOpen) return null;

    return createPortal(
        <div 
            className="fixed inset-0 z-[99999] flex items-center justify-center bg-black/50 backdrop-blur-sm"
            style={{ position: 'fixed', top: 0, left: 0, right: 0, bottom: 0 }}
        >
            <div className="absolute inset-0 z-[0]" onClick={onClose} />
            
            <div className="bg-white dark:bg-base-100 text-gray-900 dark:text-base-content rounded-2xl shadow-2xl w-full max-w-lg p-6 relative z-[10] m-4 max-h-[90vh] overflow-y-auto">
                <h3 className="font-bold text-lg mb-4 flex items-center gap-2">
                    <Bot className="w-5 h-5 text-green-600" />
                    {t('accounts.add.openai.title')}
                </h3>

                {/* Tabs */}
                <div className="bg-gray-100 dark:bg-base-200 p-1 rounded-xl mb-6 grid grid-cols-2 gap-1">
                    <button
                        className={`py-2 px-3 rounded-lg text-sm font-medium transition-all duration-200 flex items-center justify-center gap-2 ${
                            activeTab === 'web'
                                ? 'bg-white dark:bg-base-100 shadow-sm text-green-600 dark:text-green-400'
                                : 'text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'
                        }`}
                        onClick={() => setActiveTab('web')}
                    >
                        <Globe className="w-4 h-4" />
                        {t('accounts.add.openai.tabs.web')}
                    </button>
                    <button
                        className={`py-2 px-3 rounded-lg text-sm font-medium transition-all duration-200 flex items-center justify-center gap-2 ${
                            activeTab === 'api'
                                ? 'bg-white dark:bg-base-100 shadow-sm text-purple-600 dark:text-purple-400'
                                : 'text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'
                        }`}
                        onClick={() => setActiveTab('api')}
                    >
                        <Key className="w-4 h-4" />
                        {t('accounts.add.openai.tabs.api')}
                    </button>
                </div>

                <StatusAlert />

                <div className="space-y-4">
                    {/* Common Field: Email/Label */}
                    <div className="space-y-1">
                        <label className="text-xs font-semibold text-gray-500 uppercase">
                            {activeTab === 'web' ? t('accounts.add.openai.web.email') : t('accounts.add.openai.api.email')}
                        </label>
                        <input
                            type="text"
                            className="input input-bordered w-full"
                            placeholder={activeTab === 'web' ? t('accounts.add.openai.web.email_placeholder') : t('accounts.add.openai.api.email_placeholder')}
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            disabled={status === 'loading' || status === 'success'}
                        />
                    </div>

                    {/* OAuth Button for Web Tab */}
                    {activeTab === 'web' && (
                        <div className="flex justify-end">
                            <button
                                onClick={handleOAuthLogin}
                                className="text-xs flex items-center gap-1.5 text-blue-600 dark:text-blue-400 hover:underline font-medium"
                                disabled={status === 'loading'}
                            >
                                <LogIn className="w-3.5 h-3.5" />
                                <span>Sign in with OpenAI (Native)</span>
                            </button>
                        </div>
                    )}

                    {activeTab === 'web' && (
                        <>
                            <div className="alert alert-info text-xs py-2">
                                <Info className="w-4 h-4" />
                                <span>{t('accounts.add.openai.web.desc')}</span>
                            </div>
                            
                            <div className="space-y-1">
                                <label className="text-xs font-semibold text-gray-500 uppercase">
                                    {t('accounts.add.openai.web.access_token')}
                                </label>
                                <textarea
                                    className="textarea textarea-bordered w-full h-24 font-mono text-xs"
                                    placeholder={t('accounts.add.openai.web.access_token_placeholder')}
                                    value={accessToken}
                                    onChange={(e) => setAccessToken(e.target.value)}
                                    disabled={status === 'loading' || status === 'success'}
                                />
                            </div>

                            <div className="space-y-1">
                                <label className="text-xs font-semibold text-gray-500 uppercase">
                                    {t('accounts.add.openai.web.session_token')}
                                </label>
                                <input
                                    type="text"
                                    className="input input-bordered w-full font-mono text-xs"
                                    placeholder={t('accounts.add.openai.web.session_token_placeholder')}
                                    value={sessionToken}
                                    onChange={(e) => setSessionToken(e.target.value)}
                                    disabled={status === 'loading' || status === 'success'}
                                />
                            </div>
                        </>
                    )}

                    {activeTab === 'api' && (
                        <>
                            <div className="alert alert-warning text-xs py-2 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400 border-yellow-200 dark:border-yellow-900/30">
                                <Info className="w-4 h-4" />
                                <span>{t('accounts.add.openai.api.desc')}</span>
                            </div>

                            <div className="space-y-1">
                                <label className="text-xs font-semibold text-gray-500 uppercase">
                                    {t('accounts.add.openai.api.api_key')}
                                </label>
                                <input
                                    type="password"
                                    className="input input-bordered w-full font-mono"
                                    placeholder={t('accounts.add.openai.api.api_key_placeholder')}
                                    value={apiKey}
                                    onChange={(e) => setApiKey(e.target.value)}
                                    disabled={status === 'loading' || status === 'success'}
                                />
                            </div>
                        </>
                    )}
                </div>

                <div className="flex gap-3 w-full mt-8">
                    <button
                        className="flex-1 px-4 py-2.5 bg-gray-100 dark:bg-base-200 text-gray-700 dark:text-gray-300 font-medium rounded-xl hover:bg-gray-200 dark:hover:bg-base-300 transition-colors"
                        onClick={onClose}
                        disabled={status === 'success'}
                    >
                        {t('common.cancel')}
                    </button>
                    <button
                        className="flex-1 px-4 py-2.5 text-white font-medium rounded-xl shadow-md transition-all bg-green-600 hover:bg-green-700 shadow-green-200 dark:shadow-green-900/30 flex justify-center items-center gap-2"
                        onClick={handleSubmit}
                        disabled={status === 'loading' || status === 'success'}
                    >
                        {status === 'loading' ? <Loader2 className="w-4 h-4 animate-spin" /> : null}
                        {t('common.confirm')}
                    </button>
                </div>
            </div>
        </div>,
        document.body
    );
}
