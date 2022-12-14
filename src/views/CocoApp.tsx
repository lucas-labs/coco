import c from 'chalk';
import { Box, Text, useFocusManager } from 'ink';
import useStdoutDimensions from 'ink-use-stdout-dimensions';
import React, { FC, useEffect, useState } from 'react';
import { getCommitType } from '../common/config/coco.config';
import { useInput } from '../common/hooks/use-input';
import { i18n } from '../common/i18n/i18n';
import { Config, ConventionalCommitType, ValidatedValue } from '../common/types/coco.types';
import { Selector } from '../components/selector/selector';
import { BodyInput } from '../components/coco-inputs/body-input';
import { FooterInput } from '../components/coco-inputs/footer-input';
import { ScopeInput } from '../components/coco-inputs/scope-input';
import { SummaryInput } from '../components/coco-inputs/summary-input';
import { ConfirmCommit } from '../components/confirm/confirm-commit';
import { Switch } from '../components/input/switch';
import { Br } from '../components/utils/br';
import { canContinue, canGoBack } from './core/orchestration';
import { Header } from '../components/utils/header';
import { Help } from '../components/utils/help';
import { commit, CommitResult } from '../common/git/commands/commit';
import { ReviewCommit } from '../components/review/review';
import { FocusKey, Stage, stageFromFocused } from '../common/types/focus-keys.types';

export interface CocoAppProps {
    cfg: Config;
}

export const CocoApp: FC<CocoAppProps> = ({ cfg }) => {
    const [,rows] = useStdoutDimensions();
    const [typeDesc, setTypeDesc] = useState<ConventionalCommitType>();
    const [type, setType] = useState<string>();
    const [scope, setScope] = useState<ValidatedValue>({ value: '', isValid: cfg.askScope ? false : true});
    const [summary, setSummary] = useState<ValidatedValue>({ value: '', isValid: false});
    const [body, setBody] = useState<ValidatedValue>({ value: '', isValid: cfg.askBody ? false : true});
    const [footer, setFooter] = useState<ValidatedValue>({ value: '', isValid: cfg.askFooter ? false : true});
    const [breaking, setBreaking] = useState<boolean>(false);
    const [gitmoji, setGitmoji] = useState<string>('');
    const [step, setStep] = useState<FocusKey>(FocusKey.typeSelector);
    const [stage, setStage] = useState<Stage>('type_setup');
    const [prevStage, setPrevStage] = useState<Stage>();
    const [commitResult, setCommitResult] = useState<CommitResult>();
    const [commitError, setCommitError] = useState<string | Error>();
    const { focus, disableFocus, focusPrevious, focusNext } = useFocusManager();
    const sanitize = (str: string) => str.replace(/^\s+|\s+$/g, '');
    const types = cfg.types.map((type) => type.name);

    useEffect(() => {
        disableFocus();
    }, []);

    useEffect(() => {
        const newStage = stageFromFocused(step);
        setStage(newStage);
    }, [step]);

    useEffect(() => {
        if(type) {
            const desc = getCommitType(cfg.types, type);
            desc && setTypeDesc(desc); 
        }
    }, [type]);

    useInput((input, key) => {
        if(key.f2) {
            if(stage === 'help') {
                setStage(prevStage);
            } else {
                setPrevStage(stage);
                setStage('help');
            }
        }

        if(key.escape) {
            if(stage === 'help') {
                setStage(prevStage);
            }

            focus(step);
            return;
        }

        if (key.ctrl && input === 'c') {
            process.exit(0);
        }

        if(key.previous) {
            canGoBack(step) && focusPrevious();
        }

        if(key.next || key.tab) {
            if(canContinue(step, type, scope, summary, body, footer)) {
                focusNext();
            }
        }
    }, {isActive: stage !== 'review'});

    const onTypeSelected = (update: string) => {
        setType(update);
        focusNext();
    };

    const onScopeSelected = (update: ValidatedValue | string) => {
        if(typeof update === 'string') {
            setScope({ value: sanitize(update), isValid: true });
        } else {
            setScope({value: sanitize(update.value), isValid: update.isValid});
        }
        
        focusNext();
    };

    const onSummarySelected = (update: ValidatedValue) => {
        setSummary({value: sanitize(update.value), isValid: update.isValid});
        focusNext();
    };

    const onBodySelected = (update: ValidatedValue) => {
        setBody({value: sanitize(update.value), isValid: update.isValid});
        focusNext();
    };

    const onFooterSelected = (update: ValidatedValue) => {
        setFooter({value: sanitize(update.value), isValid: update.isValid});
        focusNext();
    };

    const onBreakingSelected = (v: boolean) => {
        setBreaking(v);
        focusNext();
    };

    const onStepFocus = ((v: boolean, step: FocusKey) => {
        v && setStep(step)
    });

    const compiled = () => `${type}${scope.value ? `(${scope.value})` : ''}${breaking ? '!' : ''}:${cfg.useEmoji ? ` ${typeDesc?.emoji}` : ''} ${summary.value}${body.value ? `\n\n${body.value}` : ''}${footer.value ? `\n\n${footer.value}` : ''}`;

    const onCommitConfirmed = () => {
        focus(FocusKey.reviewSelector);

        commit('.', compiled())
            .then((out) => {
                setCommitResult(out);
            })
            .catch((err) => {
                setCommitError(err);
            });
    };

    const onCanceled = () => {
        focus(FocusKey.typeSelector);
    };

    return (
        <Box minHeight={rows-1} flexDirection="column">
            <Box flexDirection="column" alignItems='center' display={stage === 'help' || stage === 'review' ? 'none' : 'flex'}>
                <Header type={stage === 'type_setup' ? 'big' : 'small'}/>
            </Box>

            <Box flexDirection="column" display={stage === 'help' ? 'flex' : 'none'} justifyContent="center" alignItems='center'>
                <Help></Help>
            </Box>

            <Box flexDirection="column" display={stage === 'type_setup' ? 'flex' : 'none'} justifyContent="center" alignItems='center'>
                <Text>{i18n('Select the type of your commit (use arrows to move around, enter to select)')}</Text>
                <Br/>
                <Selector 
                    onSelected={onTypeSelected} 
                    options={types}
                    focusKey={FocusKey.typeSelector}
                    focusChanged={(v) => onStepFocus(v, FocusKey.typeSelector)}
                /><Br/>
            </Box>

            <Box flexDirection="column" display={stage === 'message_setup' || stage === 'scope_setup' ? 'flex' : 'none'}>
                <Box alignItems="center" display={type!! ? 'flex' : 'none'} marginBottom={1} >
                    <Box>
                        <Text>{i18n("Creating a <%=type%> commit", {type: c.bold.greenBright(type)})}{scope.value ? ' ' + i18n('on scope <%=scope%>', {scope: c.blue(scope.value)}): ''} {c.dim('|')} {typeDesc?.emoji} {c.dim('>')} {c.dim.italic(i18n(typeDesc?.desc || ''))}</Text>
                    </Box>
                </Box>

                <Box display={stage === 'scope_setup' ? 'flex' : 'none'} flexDirection='column'>
                    {
                        (cfg.scopes && cfg.scopes.length) > 0 ? (
                            <Box flexDirection='column' justifyContent="center" alignItems='center'>
                                <Text>{i18n('Select the scope of your commit (use arrows to move around, enter to select)')}</Text><Br/>                                
                                <Selector onSelected={onScopeSelected} options={cfg.scopes} focusKey={FocusKey.scopeSelector} focusChanged={(v) => onStepFocus(v, FocusKey.scopeSelector)}/>
                            </Box>
                        ) : (
                            <ScopeInput focusChanged={(v) => onStepFocus(v, FocusKey.scopeSelector)}   onSelected={onScopeSelected}   display={type !== undefined} focusable={cfg.askScope} />
                        )
                    }
                    <Br/>
                </Box>

                <Box display={stage === 'message_setup' ? 'flex' : 'none'} flexDirection='column'>
                    <SummaryInput focusChanged={(v) => onStepFocus(v, FocusKey.summarySelector)} onSelected={onSummarySelected} display={scope.isValid}/><Br/>
                    <BodyInput    focusChanged={(v) => onStepFocus(v, FocusKey.bodySelector)}    onSelected={onBodySelected}    display={summary.isValid} focusable={cfg.askBody} /><Br/>
                    <FooterInput  focusChanged={(v) => onStepFocus(v, FocusKey.footerSelector)}  onSelected={onFooterSelected}  display={body.isValid}    focusable={cfg.askFooter} />
                </Box>
            </Box>

            <Box flexDirection="column" display={stage === 'breaking' ? 'flex' : 'none'}>
                <Switch
                    focusKey={FocusKey.breakingSelector}
                    focusChanged={(v) => onStepFocus(v, FocusKey.breakingSelector)}
                    onSelected={(v) => onBreakingSelected(v)}
                    onChange={setBreaking}
                />
            </Box>
            
            <Box flexDirection="column" display={stage === 'confirm' ? 'flex' : 'none'}>
                <ConfirmCommit  
                    focusChanged={(v) => onStepFocus(v, FocusKey.confirmSelector)} 
                    type={type}
                    scope={scope.value}
                    summary={summary.value}
                    body={body.value}
                    footer={footer.value}
                    breaking={breaking}
                    gitmoji={cfg.useEmoji ? typeDesc?.emoji : undefined}
                    onCommitConfirmed={onCommitConfirmed}
                    onCanceled={onCanceled}
                ></ConfirmCommit>
            </Box>

            <Box flexDirection="column" display={stage === 'review' ? 'flex' : 'none'}>
                <ReviewCommit 
                    result={commitResult} 
                    err={commitError}
                    focusChanged={(v) => onStepFocus(v, FocusKey.reviewSelector)}
                />
            </Box>
        </Box>
    );
};
