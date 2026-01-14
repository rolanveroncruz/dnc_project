import {
  AfterViewInit, Component, Input, OnChanges, SimpleChanges,
  ViewChild, ViewEncapsulation, inject, EventEmitter, Output, TemplateRef
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormBuilder, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatSort, MatSortModule} from '@angular/material/sort';
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { TableColumn } from './table-interfaces';
import {MatChip, MatChipSet} from '@angular/material/chips'; // Import from where you defined it

@Component({
  selector: 'app-generic-data-table', // Renamed
  standalone: true,
  imports: [
    CommonModule, ReactiveFormsModule, MatTableModule, MatSortModule,
    MatPaginatorModule, MatFormFieldModule, MatInputModule,
    MatSelectModule, MatButtonModule, MatIconModule, MatChipSet, MatChip,
  ],
  templateUrl: './generic-data-table-component.html',
  styleUrls: ['./generic-data-table-component.scss'],
  encapsulation: ViewEncapsulation.None // Optional: helps with generic styles
})
export class GenericDataTableComponent<T> implements AfterViewInit, OnChanges {
  @ViewChild('defaultCell', { static: true }) defaultCell!: TemplateRef<any>;
  @ViewChild('dateCell', { static: true }) dateCell!: TemplateRef<any>;
  @ViewChild('datetimeCell', { static: true }) dateTimeCell!: TemplateRef<any>;
  @ViewChild('chipsCell', { static: true }) chipsCell!: TemplateRef<any>;
  // --- INPUTS ---
  @Input({ required: true }) data: T[] = [];
  @Input({ required: true }) columnDefs: TableColumn[] = [];
  @Input() showAddButton = true;

  @Output() rowClicked = new EventEmitter<T>();
  @Output() addClicked = new EventEmitter<void>();

  // New Configurable paginator inputs
  @Input() pageSize= 15;
  @Input() pageSizeOptions: number[] = [5, 10, 25, 50, 100];

  // Keys that should show as dropdown filters (e.g. ['role', 'status'])
  @Input() filterSelectKeys: string[] = [];

  // --- STATE ---
  dataSource = new MatTableDataSource<T>([]);
  displayedColumns: string[] = [];

  // Dynamic filter options (Map of key -> unique values)
  filterOptions: { [key: string]: string[] } = {};

  private fb = inject(FormBuilder);
  form: FormGroup = this.fb.group({
    q: [''],
    sortBy: [''],
    sortDir: ['asc'],
    filters: this.fb.group({}) // Dynamic nested form group for select filters
  });

  @ViewChild(MatSort) sort!: MatSort;
  @ViewChild(MatPaginator) paginator!: MatPaginator;

  constructor() {
    this.form.valueChanges.subscribe(() => {
      this.applyFilter();
      this.applySortFromToolbar();
    });
  }
  onRowClicked(row: T) {
    console.log("Row clicked:", row);
    this.rowClicked.emit(row);
  }
  onAddClicked() {
    this.addClicked.emit();
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['data'] ) {
      const idx = (this.data??[]).findIndex(x=>x==null);
      if (idx !==-1){
        console.error("Generic Data Table: data contains null/undefined at index", idx, this.data[idx]);
      }
      this.dataSource.data = (this.data || []).filter((x): x is T => x != null);
      this.generateFilterOptions(); // Recalculate unique values for dropdowns
    }
    if (changes['pageSize'] && this.paginator){
      this.paginator.pageSize = this.pageSize;
      this.paginator.firstPage();
    }

    if (changes['columnDefs']) {
      this.displayedColumns = this.columnDefs.map(c => c.key);
      // Set default sort by first column if not set
      if (!this.form.get('sortBy')?.value && this.columnDefs.length > 0) {
        this.form.patchValue({ sortBy: this.columnDefs[0].key }, { emitEvent: false });
      }
    }

    if (changes['filterSelectKeys']) {
      this.rebuildFilterForm();
    }
  }

  ngAfterViewInit(): void {
    this.dataSource.sort = this.sort;
    this.dataSource.paginator = this.paginator;
    this.paginator.pageSize = this.pageSize;
    this.setupFilterPredicate();

    setTimeout(() => {
      this.applySortFromToolbar();

    })
  }

  /**
   * Rebuilds the nested 'filters' FormGroup based on input keys
   */
  private rebuildFilterForm(): void {
    const filtersGroup = this.form.get('filters') as FormGroup;
    // Clear existing controls
    Object.keys(filtersGroup.controls).forEach(key => filtersGroup.removeControl(key));

    // Add new controls
    this.filterSelectKeys.forEach(key => {
      filtersGroup.addControl(key, this.fb.control(''));
    });
  }

  /**
   * Extract unique values from data to populate dropdowns automatically
   */
  private generateFilterOptions(): void {
    this.filterSelectKeys.forEach(key => {
      // Get unique values, filter out nulls/undefined
      const unique = [...new Set(this.data.map((item: any) => item[key]))].filter(x => x);
      this.filterOptions[key] = unique.sort();
    });
  }

  private setupFilterPredicate() {
    this.dataSource.filterPredicate = (data: T, filterJson: string) => {
      const f = JSON.parse(filterJson);
      const searchText = (f.q || '').toLowerCase();
      const specificFilters = f.filters || {};

      // 1. Check Global Search (q) against all displayed columns
      const matchesSearch = !searchText || this.columnDefs.some(col => {
        const val = (data as any)[col.key];
        return String(val ?? '').toLowerCase().includes(searchText);
      });

      // 2. Check Specific Dropdown Filters
      const matchesFilters = Object.keys(specificFilters).every(key => {
        const requiredValue = specificFilters[key];
        if (!requiredValue) return true; // Ignored if the dropdown is empty
        return String((data as any)[key]) === String(requiredValue);
      });

      return matchesSearch && matchesFilters;
    };
  }

  clearSearch(): void {
    this.form.patchValue({ q: '' });
  }

  resetAll(): void {
    // Reset core form
    this.form.patchValue({
      q: '',
      sortBy: this.columnDefs[0]?.key,
      sortDir: 'asc',
    });

    // Reset dynamic filters
    const filtersGroup = this.form.get('filters') as FormGroup;
    filtersGroup.reset();
  }

  private applyFilter(): void {
    const v = this.form.getRawValue();
    this.dataSource.filter = JSON.stringify({
      q: v.q,
      filters: v.filters // Passes the nested object { role: '...', status: '...' }
    });
    this.dataSource.paginator?.firstPage();
  }

  private applySortFromToolbar(): void {
    if (!this.sort) return;
    const { sortBy, sortDir } = this.form.getRawValue();
    if(sortBy) {
      this.sort.active = sortBy;
      this.sort.direction = sortDir;
      this.sort.sortChange.emit({ active: sortBy, direction: sortDir });
    }
  }
  protected toDate(value: unknown) :Date | null {
    if (value==null || value==="") return null;

    if (value instanceof Date) return value;
    if (typeof value === "number") {
      const d = new Date(value);
      return isNaN(d.getTime()) ? null : d;
    }
    if (typeof value === "string") {
      const normalized = value.includes(' ') ? value.replace(' ', 'T') : value;
      const d = new Date(normalized);
      return isNaN(d.getTime()) ? null : d;
    }
    return null;
  }
  getCellTemplate(col:TableColumn): TemplateRef<any>{

    switch(col.cellTemplateKey){
      case 'date': return this.dateCell;
      case 'datetime': return this.dateTimeCell;
      case 'chips': return this.chipsCell;
      case 'default': return this.defaultCell;
    }
    return this.defaultCell;

  }
  chipClass( tag:unknown): string{
    const t=String(tag).toLowerCase();
    if (t=='create') return 'tag-create';
    if (t=='read') return 'tag-read';
    if (t=='update') return 'tag-update';
    return 'tag-other';
  }
}

