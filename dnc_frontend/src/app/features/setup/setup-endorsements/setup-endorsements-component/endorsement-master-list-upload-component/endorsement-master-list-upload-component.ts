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
import {ExistingMasterListMeta, MasterListPreview } from './data-types';
import {MatDivider} from '@angular/material/list';
import {MatDialog, MatDialogModule} from '@angular/material/dialog';
import {
    SimpleConfirmDialogComponent
} from '../../../../../components/simple-confirm-dialog-component/simple-confirm-dialog-component';
import {EndorsementMasterListService} from '../../../../../api_services/endorsement-master-list-service';

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
        MatDialogModule,
    ],
    templateUrl: './endorsement-master-list-upload-component.html',
    styleUrls: ['./endorsement-master-list-upload-component.scss'],
})
/*
  The EndorsementMasterListUploadComponent is responsible for uploading a master list for an endorsement.
  It handles the upload process, previewing the uploaded file, and saving the master list.
  When a file has not yet been uploaded, it allows continuously uploading a new file to add members.


 */
export class EndorsementMasterListUploadComponent {
    private readonly destroyRef = inject(DestroyRef);
    private readonly endorsementMasterListService = inject(EndorsementMasterListService);
    private readonly dialog = inject(MatDialog);

    @Input({ required: true }) endorsementId!: number | null;
    @Input({ required: true }) enabled!: boolean;
    @Input() existing: ExistingMasterListMeta | null = null;

    @Output() saved = new EventEmitter<void>();
    @Output() cleared = new EventEmitter<void>();
    @Input() onViewExisting: (()=>void) | null = null;

    readonly hasValidEndorsementId = computed(()=> {
        const id = this.endorsementId;
        return typeof id==='number' && Number.isFinite(id) && id > 0;
    });


    readonly ui = signal<UploadUiState>('idle');
    readonly errorMsg = signal<string | null>(null);

    readonly preview = signal<MasterListPreview | null>(null);



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

        // reset input so selecting the same file again triggers a change
        input.value = '';

        this.upload(file);
    }

    upload(file: File) {
        this.errorMsg.set(null);
        this.preview.set(null);

        if (!this.enabled) return;
        if (this.endorsementId == null) {
            this.errorMsg.set('Please save the endorsement first before uploading a master list.');
            this.ui.set('error');
            return;
        }

        this.ui.set('uploading');

        this.endorsementMasterListService
            .uploadEndorsementMasterList(this.endorsementId, file)
            .pipe(
                tap((p) => {
                    this.preview.set(null);
                    this.ui.set('saved');
                    this.saved.emit();
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


    viewExisting() {
        this.onViewExisting?.();

    }

    removeExisting() {
        const eid = this.endorsementId;
        if (eid == null) return;

        const dialogRef=this.dialog.open(SimpleConfirmDialogComponent, {
            width: '400px',
            data: {
                title: 'Confirm Deletion',
                message: 'Are you sure you want to delete the existing master list?',
                confirmText: 'Delete',
                cancelText: 'Cancel',
            },
        });

        dialogRef.afterClosed()
            .pipe(
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe((confirmed)=>{
                if (!confirmed) return;

                this.ui.set('saving');
                this.errorMsg.set(null);
                this.endorsementMasterListService
                    .deleteEndorsementMasterList(eid)
                    .pipe(
                        tap(()=>{
                            this.ui.set('saved');
                            this.preview.set(null);
                            this.cleared.emit();
                        }),
                        catchError((err)=>{
                            console.error('Master list delete failed:', err);
                            this.errorMsg.set('Delete failed. Please try again.');
                            this.ui.set('error');
                            return of(null);
                        }),
                        finalize(()=>{
                            if (this.ui() === 'saving') this.ui.set('idle');
                        }),
                        takeUntilDestroyed(this.destroyRef),
                    )
                    .subscribe();
            });


    }

}
