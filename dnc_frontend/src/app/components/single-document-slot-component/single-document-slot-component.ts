import {
    Component,
    DestroyRef,
    EventEmitter,
    Input,
    Output,
    computed,
    inject,
    signal, numberAttribute,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import {HttpClient, HttpHeaders} from '@angular/common/http';

import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatDividerModule } from '@angular/material/divider';

import { catchError, finalize, of } from 'rxjs';
import { SingleDocumentUploadService, StoredDocumentMeta } from '../../api_services/single-document-upload-service';
import {environment} from '../../../environments/environment';
import {LoginService} from '../../login.service';

@Component({
    selector: 'app-single-document-slot',
    standalone: true,
    imports: [
        CommonModule,
        MatIconModule,
        MatButtonModule,
        MatTooltipModule,
        MatProgressBarModule,
        MatDividerModule,
    ],
    templateUrl: './single-document-slot-component.html',
    styleUrl: './single-document-slot-component.scss',
})
export class SingleDocumentSlotComponent {
    private readonly api = inject(SingleDocumentUploadService);
    private readonly destroyRef = inject(DestroyRef);
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    /** The owning entity id (e.g., dentistId) */
    @Input({transform: numberAttribute, required: true}) dentistId!: number;

    /** Label shown above the slot */
    @Input() label = 'Document';

    /** Optional: disable everything (e.g., while the page is saving) */
    @Input() disabled = false;

    /** Set the filename  */
    @Input() set existingFilename(filename: string | null) {
        if (!filename) {
            this.meta.set(null);
            return;
        }
        console.log("Setting filename to", filename);
        this.meta.set({
            id:0,
            file_name: filename,
            file_path: filename,
            content_type: 'application/octet-stream',
            size_bytes: 0,
            updated_at: new Date(0).toISOString()
        });
    }

    /**
     * Emits whenever the “pending selected file” changes.
     * Parent can use this to show “Unsaved changes” on the page, etc.
     */
    @Output() pendingChanged = new EventEmitter<boolean>();

    /**
     * Emits after a successful upload/replace.
     */
    @Output() uploaded = new EventEmitter<StoredDocumentMeta>();

    /**
     * Emits after deletion.
     */
    @Output() deleted = new EventEmitter<void>();

    // --- state
    readonly meta = signal<StoredDocumentMeta | null>(null);
    readonly loadState = signal<'idle' | 'loading' | 'error'>('idle');
    readonly busy = signal(false);

    // Pending file (not uploaded yet)
    readonly pendingFile = signal<File | null>(null);

    readonly hasPending = computed(() => !!this.pendingFile());
    readonly canUploadNow = computed(() => !this.disabled && !this.busy() && !!this.pendingFile());

    /** Call this when the parent loads the page */
    refresh(): void {
    }

    /** Parent calls this after saving other fields to commit the upload, if any */
    commitPendingUpload(): void {
        const file = this.pendingFile();
        if (!file || this.disabled || this.busy()) return;

        this.busy.set(true);
        console.log("calling api.uploadReplace");

        this.api.uploadReplace(this.dentistId, file).pipe(
            takeUntilDestroyed(this.destroyRef),
            finalize(() => this.busy.set(false))
        ).subscribe({
            next: (m) => {
                this.meta.set(m);
                this.pendingFile.set(null);
                this.pendingChanged.emit(false);
                this.uploaded.emit(m);
            },
            error: () => {
                // keep pendingFile so the user can try again
            },
        });
    }

    clearPendingSelection(): void {
        this.pendingFile.set(null);
        this.pendingChanged.emit(false);
    }

    onFilePicked(ev: Event): void {
        const input = ev.target as HTMLInputElement;
        const file = input.files?.[0] ?? null;

        // allow re-picking the same file later by clearing the input value
        input.value = '';

        if (!file) return;

        this.pendingFile.set(file);
        this.pendingChanged.emit(true);
    }

    deleteExisting(): void {
        if (this.disabled || this.busy()) return;

        this.busy.set(true);

        this.api.delete(this.dentistId).pipe(
            takeUntilDestroyed(this.destroyRef),
            finalize(() => this.busy.set(false))
        ).subscribe({
            next: () => {
                this.meta.set(null);
                this.deleted.emit();
            },
            error: () => {},
        });
    }

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }
    download(): void {
        const meta = this.meta();
        if (!meta) return;

        const encodedName = encodeURIComponent(meta.file_name);
        const url = `${environment.apiUrl}/api/dentists/${this.dentistId}/contract-file/${encodedName}`;

        // Open a blank tab immediately (keeps popup blockers happy)
        const tab = window.open('', '_blank', 'noopener');

        this.http.get(url, { responseType: 'blob', headers: this.authHeaders() }).subscribe({
            next: (blob) => {
                const blobUrl = URL.createObjectURL(blob);
                if (tab) tab.location.href = blobUrl;
                else window.open(blobUrl, '_blank', 'noopener');
                setTimeout(() => URL.revokeObjectURL(blobUrl), 60_000);
            },
            error: (err) => {
                if (tab) tab.close();
                console.error('Download failed', err);
            },
        });
    }

}
