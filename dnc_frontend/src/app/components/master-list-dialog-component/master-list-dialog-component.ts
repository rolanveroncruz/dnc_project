import {
    Component,
    Input,
    Output,
    EventEmitter,
    OnChanges,
    SimpleChanges,
    ViewChild,
    ChangeDetectionStrategy,
    inject,
    OnInit,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatDialogModule, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';

import { GenericDataTableComponent } from '../generic-data-table-component/generic-data-table-component';
import { TableColumn } from '../generic-data-table-component/table-interfaces';

export interface EndorsementMasterListMemberResponse {
    file_name: string;
    master_list_member_id: number;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
    is_active: boolean;
}

interface MasterListDialogRow extends EndorsementMasterListMemberResponse {
    account_number_last_name: string;
}
interface MasterListDialogData {
    members: EndorsementMasterListMemberResponse[];
}

@Component({
    selector: 'app-master-list-dialog',
    standalone: true,
    imports: [
        CommonModule,
        MatDialogModule,
        MatButtonModule,
        GenericDataTableComponent,
    ],
    templateUrl: './master-list-dialog-component.html',
    styleUrls: ['./master-list-dialog-component.scss'],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MasterListDialogComponent implements OnInit, OnChanges {
    @Input({ required: true }) members: EndorsementMasterListMemberResponse[] = [];
    private readonly dialogData = inject<MasterListDialogData | null>(MAT_DIALOG_DATA, { optional: true });

    /**
     * Parent can bind to this:
     * (isActiveClicked)="onIsActiveClicked($event)"
     */
    @Output() isActiveClicked = new EventEmitter<EndorsementMasterListMemberResponse>();

    @ViewChild(GenericDataTableComponent)
    table?: GenericDataTableComponent<MasterListDialogRow>;

    protected rows: MasterListDialogRow[] = [];

    protected readonly columnDefs: TableColumn[] = [
        { key: 'file_name', label: 'File Name', },
        { key: 'account_number', label: 'Account Number', },
        {key: 'last_name', label: 'Last Name'},
        { key: 'first_name', label: 'First Name', },
        { key: 'middle_name', label: 'Middle Name', sortable: true, },
        { key: 'is_active', label: 'Active', sortable: true, cellTemplateKey: 'check', },
    ];

    protected readonly dialogRef =
        inject<MatDialogRef<MasterListDialogComponent> | null>(MatDialogRef, {
            optional: true,
        });

    ngOnInit(): void {
        if (this.dialogData?.members) {
            this.members = this.dialogData.members;
            this.rebuildRows();
        }
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (changes['members']) {
            this.rebuildRows();

        }
    }
    private rebuildRows():void{
        this.rows = (this.members?? []).map((m)=>({
            ...m,
            account_number_last_name: `${m.account_number} - ${m.last_name}`,
        }));
    }

    protected close(): void {
        this.dialogRef?.close();
    }

    protected onTableClick(event: MouseEvent): void {
        const target = event.target as HTMLElement | null;
        if (!target || !this.table) return;

        const td = target.closest('td');
        if (!td) return;

        if (!td.classList.contains('mat-column-is_active')) return;

        const tr = td.closest('tr');
        if (!tr) return;

        const rowIndexAttr = tr.getAttribute('data-row-index');
        if (rowIndexAttr == null) return;

        const rowIndex = Number(rowIndexAttr);
        if (Number.isNaN(rowIndex)) return;

        const renderedRows = this.getRenderedRows();
        const clickedRow = renderedRows[rowIndex];
        if (!clickedRow) return;

        this.isActiveClicked.emit({
            file_name: clickedRow.file_name,
            master_list_member_id: clickedRow.master_list_member_id,
            account_number: clickedRow.account_number,
            last_name: clickedRow.last_name,
            first_name: clickedRow.first_name,
            middle_name: clickedRow.middle_name,
            is_active: clickedRow.is_active,
        });
    }

    /**
     * Reconstruct the rows currently rendered by GenericDataTableComponent
     * so we can map the DOM row index -> actual row object.
     */
    private getRenderedRows(): MasterListDialogRow[] {
        if (!this.table) return [];

        let rows = [...this.table.dataSource.filteredData];

        const sort = this.table.sort;
        if (sort?.active && sort.direction) {
            rows = this.table.dataSource.sortData(rows, sort);
        }

        const paginator = this.table.paginator;
        if (paginator) {
            const start = paginator.pageIndex * paginator.pageSize;
            const end = start + paginator.pageSize;
            rows = rows.slice(start, end);
        }

        return rows;
    }
}
