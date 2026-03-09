import { CommonModule } from '@angular/common';
import {
    Component,
    ChangeDetectionStrategy,
    DestroyRef,
    EventEmitter,
    Input,
    Output,
    computed,
    inject,
    signal, effect,
} from '@angular/core';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatIconModule } from '@angular/material/icon';
import { MatTableModule } from '@angular/material/table';
import { MatChipsModule } from '@angular/material/chips';
import { MatExpansionModule } from '@angular/material/expansion';
import { catchError, finalize, of, tap } from 'rxjs';
import {EndorsementService} from '../../../../../api_services/endorsement-service';
import {ExistingMasterListMeta, MasterListPreview, MasterListIssue} from './data-types';
import {MatDivider} from '@angular/material/list';

type UploadUiState = 'idle' | 'uploading' | 'preview' | 'saving' | 'saved' | 'error';

@Component({
    selector: 'app-endorsement-masterlist-upload',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [
        CommonModule,
        MatCardModule,
        MatButtonModule,
        MatProgressBarModule,
        MatIconModule,
        MatTableModule,
        MatChipsModule,
        MatExpansionModule,
        MatDivider,
    ],
    templateUrl: './endorsement-master-list-upload-component.html',
    styleUrls: ['./endorsement-master-list-upload-component.scss'],
})
export class EndorsementMasterListUploadComponent {
    private readonly destroyRef = inject(DestroyRef);
    private readonly endorsementService = inject(EndorsementService);

    // ✅ Inputs: keep simple for parent
    @Input({ required: true }) endorsementId!: number | null;
    @Input({ required: true }) enabled!: boolean;
    @Input() existing: ExistingMasterListMeta | null = null;

    // ✅ Outputs: parent can update meta
    @Output() saved = new EventEmitter<ExistingMasterListMeta>();
    @Output() cleared = new EventEmitter<void>();

    readonly hasValidEndorsementId = computed(()=> {
        const id = this.endorsementId;
        return typeof id==='number' && Number.isFinite(id) && id > 0;
    });

    readonly isLocked = computed(()=> !this.enabled || !this.hasValidEndorsementId());

    readonly ui = signal<UploadUiState>('idle');
    readonly errorMsg = signal<string | null>(null);

    // ✅ Preview returned by server (parse-only)
    readonly preview = signal<MasterListPreview | null>(null);

    // ✅ derived display
    readonly hasErrors = computed(() => (this.preview()?.issues ?? []).some(i => i.severity === 'error'));
    readonly errorCount = computed(() => (this.preview()?.issues ?? []).filter(i => i.severity === 'error').length);
    readonly warnCount = computed(() => (this.preview()?.issues ?? []).filter(i => i.severity === 'warn').length);

    // ✅ table columns: update once you decide your sheet schema
    readonly displayedColumns = ['member_id', 'full_name', 'birthdate'];

    constructor(){
        effect(()=>{
            if (!this.enabled) {
                this.preview.set(null)
                this.errorMsg.set(null)
                if (this.ui() !== 'idle') this.ui.set('idle');
            }
        })
    }

    onPickFile(input: HTMLInputElement) {
        const file = input.files?.[0] ?? null;
        if (!file) return;

        // reset input so selecting same file again triggers change
        input.value = '';

        this.uploadForPreview(file);
    }

    uploadForPreview(file: File) {
        this.errorMsg.set(null);
        this.preview.set(null);

        if (!this.enabled) return;
        if (this.endorsementId == null) {
            // ✅ Important: must have an endorsement id to attach later.
            this.errorMsg.set('Please save the endorsement first before uploading a master list.');
            this.ui.set('error');
            return;
        }

        this.ui.set('uploading');

        this.endorsementService
            .previewEndorsementMasterList(this.endorsementId, file)
            .pipe(
                tap((p) => {
                    this.preview.set(p);
                    this.ui.set('preview');
                }),
                catchError((err) => {
                    console.error('Master list preview failed:', err);
                    this.errorMsg.set('Preview failed. Please check the file format and try again.');
                    this.ui.set('error');
                    return of(null);
                }),
                finalize(() => {
                    if (this.ui() === 'uploading') this.ui.set('idle');
                }),
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe();
    }

    commit() {
        const eid = this.endorsementId;
        const p = this.preview();
        if (!this.enabled || eid == null || !p) return;
        if (this.hasErrors()) return;

        this.ui.set('saving');
        this.errorMsg.set(null);

        this.endorsementService
            .commitEndorsementMasterList(eid, p.temp_upload_id)
            .pipe(
                tap((meta) => {
                    this.ui.set('saved');
                    this.saved.emit(meta);
                }),
                catchError((err) => {
                    console.error('Master list commit failed:', err);
                    this.errorMsg.set('Save failed. Please try again.');
                    this.ui.set('error');
                    return of(null);
                }),
                finalize(() => {
                    if (this.ui() === 'saving') this.ui.set('preview');
                }),
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe();
    }

    replace() {
        // ✅ Just reset local preview and let user upload again
        this.preview.set(null);
        this.errorMsg.set(null);
        this.ui.set('idle');
    }

    removeExisting() {
        const eid = this.endorsementId;
        if (eid == null) return;

        this.ui.set('saving');
        this.errorMsg.set(null);

        this.endorsementService
            .deleteEndorsementMasterList(eid)
            .pipe(
                tap(() => {
                    this.ui.set('idle');
                    this.preview.set(null);
                    this.cleared.emit();
                }),
                catchError((err) => {
                    console.error('Delete master list failed:', err);
                    this.errorMsg.set('Could not remove the master list.');
                    this.ui.set('error');
                    return of(null);
                }),
                finalize(() => {
                    if (this.ui() === 'saving') this.ui.set('idle');
                }),
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe();
    }

    // ✅ helper for template
    trackIssue = (_: number, it: MasterListIssue) => `${it.severity}-${it.row ?? 'na'}-${it.message}`;
}
