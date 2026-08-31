import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SearchDentist } from './search-dentist';

describe('FindDentistComponent', () => {
  let component: SearchDentist;
  let fixture: ComponentFixture<SearchDentist>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SearchDentist]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SearchDentist);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
